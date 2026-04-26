use eframe::egui;
use feature_registry::{FeatureRegistry, RegisteredFeature, default_workspace_root};
use feature_runner::{open_path_in_file_browser, run_feature_tests};

use crate::panels::{activity_log, demo_panel, feature_detail, feature_list, test_runner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindFilter {
    All,
    Ui,
    Logic,
    Workflow,
}

impl KindFilter {
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Ui => "ui",
            Self::Logic => "logic",
            Self::Workflow => "workflow",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeatureRow {
    pub id: String,
    pub category: String,
    pub name: String,
    pub status: String,
    pub summary: String,
}

pub struct FeatureLabApp {
    pub(crate) registry: FeatureRegistry,
    pub(crate) selected_feature_id: Option<String>,
    pub(crate) search_query: String,
    pub(crate) kind_filter: KindFilter,
    pub(crate) test_output: String,
    pub(crate) activity_log: Vec<String>,
    pub(crate) command_palette_demo_query: String,
    pub(crate) command_palette_demo_selected_index: usize,
    pub(crate) command_palette_last_activation: String,
    pub(crate) activity_stream_demo_query: String,
    pub(crate) activity_stream_demo_selected_index: usize,
    pub(crate) activity_stream_last_activation: String,
    pub(crate) validation_pipeline_demo_use_invalid: bool,
    pub(crate) left_rail_demo_selected_index: usize,
    pub(crate) left_rail_demo_collapsed: bool,
    pub(crate) left_rail_last_activation: String,
}

impl FeatureLabApp {
    pub fn bootstrap() -> Self {
        match FeatureRegistry::discover() {
            Ok(registry) => {
                let first_feature = registry
                    .features()
                    .first()
                    .map(|feature| feature.manifest.id.clone());
                Self {
                    registry,
                    selected_feature_id: first_feature,
                    search_query: String::new(),
                    kind_filter: KindFilter::All,
                    test_output: "No tests run yet.".into(),
                    activity_log: vec!["Feature registry loaded successfully.".into()],
                    command_palette_demo_query: String::new(),
                    command_palette_demo_selected_index: 0,
                    command_palette_last_activation: "No command activated yet.".into(),
                    activity_stream_demo_query: String::new(),
                    activity_stream_demo_selected_index: 0,
                    activity_stream_last_activation: "No activity entry activated yet.".into(),
                    validation_pipeline_demo_use_invalid: false,
                    left_rail_demo_selected_index: 0,
                    left_rail_demo_collapsed: false,
                    left_rail_last_activation: "No navigation item activated yet.".into(),
                }
            }
            Err(error) => Self {
                registry: FeatureRegistry::empty(default_workspace_root()),
                selected_feature_id: None,
                search_query: String::new(),
                kind_filter: KindFilter::All,
                test_output: "No tests run yet.".into(),
                activity_log: vec![format!("Failed to load registry: {error}")],
                command_palette_demo_query: String::new(),
                command_palette_demo_selected_index: 0,
                command_palette_last_activation: "No command activated yet.".into(),
                activity_stream_demo_query: String::new(),
                activity_stream_demo_selected_index: 0,
                activity_stream_last_activation: "No activity entry activated yet.".into(),
                validation_pipeline_demo_use_invalid: false,
                left_rail_demo_selected_index: 0,
                left_rail_demo_collapsed: false,
                left_rail_last_activation: "No navigation item activated yet.".into(),
            },
        }
    }

    pub(crate) fn selected_feature(&self) -> Option<RegisteredFeature> {
        self.selected_feature_id
            .as_deref()
            .and_then(|feature_id| self.registry.get(feature_id))
            .cloned()
    }

    pub(crate) fn feature_rows(&self) -> Vec<FeatureRow> {
        self.registry
            .features()
            .iter()
            .filter(|feature| self.matches_filter(feature))
            .map(|feature| FeatureRow {
                id: feature.manifest.id.clone(),
                category: feature.category(),
                name: feature.manifest.name.clone(),
                status: feature.manifest.status.to_string(),
                summary: feature.manifest.summary.clone(),
            })
            .collect()
    }

    pub(crate) fn select_feature(&mut self, feature_id: String) {
        self.selected_feature_id = Some(feature_id);
    }

    pub(crate) fn push_log(&mut self, message: impl Into<String>) {
        self.activity_log.push(message.into());
    }

    pub(crate) fn run_selected_tests(&mut self) {
        let Some(feature_id) = self.selected_feature_id.clone() else {
            self.push_log("No feature selected for test run.");
            return;
        };
        match run_feature_tests(&self.registry, &feature_id) {
            Ok(output) => {
                let status_code = output.status_code;
                let combined = if output.stderr.trim().is_empty() {
                    output.stdout
                } else {
                    format!("{}\n--- stderr ---\n{}", output.stdout, output.stderr)
                };
                self.test_output = combined;
                self.push_log(format!(
                    "Ran tests for {feature_id} with status {status_code}"
                ));
            }
            Err(error) => {
                self.test_output = error.to_string();
                self.push_log(format!("Test run failed for {feature_id}: {error}"));
            }
        }
    }

    pub(crate) fn open_selected_feature_folder(&mut self) {
        let Some(feature) = self.selected_feature() else {
            self.push_log("No feature selected to open.");
            return;
        };
        match open_path_in_file_browser(&feature.feature_dir) {
            Ok(output) => self.push_log(format!(
                "Opened feature folder for {} via {}",
                feature.manifest.id, output.command
            )),
            Err(error) => self.push_log(format!(
                "Open folder failed for {}: {error}",
                feature.manifest.id
            )),
        }
    }

    fn matches_filter(&self, feature: &RegisteredFeature) -> bool {
        let kind_match = match self.kind_filter {
            KindFilter::All => true,
            KindFilter::Ui => feature.manifest.id.starts_with("ui."),
            KindFilter::Logic => feature.manifest.id.starts_with("logic."),
            KindFilter::Workflow => feature.manifest.id.starts_with("workflow."),
        };
        if !kind_match {
            return false;
        }
        let search = self.search_query.trim().to_lowercase();
        if search.is_empty() {
            return true;
        }
        let haystack = format!(
            "{} {} {} {}",
            feature.manifest.id,
            feature.manifest.name,
            feature.manifest.summary,
            feature.manifest.tags.join(" ")
        )
        .to_lowercase();
        haystack.contains(&search)
    }
}

impl eframe::App for FeatureLabApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search");
                ui.text_edit_singleline(&mut self.search_query);
                egui::ComboBox::from_label("Filter")
                    .selected_text(self.kind_filter.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.kind_filter,
                            KindFilter::All,
                            KindFilter::All.label(),
                        );
                        ui.selectable_value(
                            &mut self.kind_filter,
                            KindFilter::Ui,
                            KindFilter::Ui.label(),
                        );
                        ui.selectable_value(
                            &mut self.kind_filter,
                            KindFilter::Logic,
                            KindFilter::Logic.label(),
                        );
                        ui.selectable_value(
                            &mut self.kind_filter,
                            KindFilter::Workflow,
                            KindFilter::Workflow.label(),
                        );
                    });
                if ui.button("Run Tests").clicked() {
                    self.run_selected_tests();
                }
                if ui.button("Open Feature Folder").clicked() {
                    self.open_selected_feature_folder();
                }
            });
        });

        egui::SidePanel::left("feature_list")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| feature_list::show(ui, self));

        egui::SidePanel::right("feature_detail")
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| feature_detail::show(ui, self));

        egui::TopBottomPanel::bottom("test_output")
            .resizable(true)
            .default_height(220.0)
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    test_runner::show(&mut columns[0], self);
                    activity_log::show(&mut columns[1], self);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| demo_panel::show(ui, self));
    }
}
