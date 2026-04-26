use eframe::egui;
use feature_registry::{FeatureRegistry, RegisteredFeature, default_workspace_root};
use feature_runner::{open_path_in_file_browser, run_feature_tests};
use theme_editor::ThemeEditor;

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

#[derive(Debug, Clone, Copy)]
pub struct FeatureCounts {
    pub total: usize,
    pub ui: usize,
    pub logic: usize,
    pub workflow: usize,
}

pub struct FeatureLabApp {
    pub(crate) registry: FeatureRegistry,
    pub(crate) selected_feature_id: Option<String>,
    pub(crate) search_query: String,
    pub(crate) kind_filter: KindFilter,
    pub(crate) glass_enabled: bool,
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
    pub(crate) theme_editor_demo: ThemeEditor,
    pub(crate) theme_editor_last_action: String,
}

impl FeatureLabApp {
    pub fn configure_platform_visuals(ctx: &egui::Context) {
        Self::apply_visuals(ctx, cfg!(target_os = "macos"));
    }

    fn apply_visuals(ctx: &egui::Context, glass_enabled: bool) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();

        if glass_enabled {
            style.visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(18, 20, 24, 102);
            style.visuals.window_fill = egui::Color32::from_rgba_unmultiplied(28, 31, 36, 140);
            style.visuals.faint_bg_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18);
            style.visuals.extreme_bg_color = egui::Color32::from_rgba_unmultiplied(8, 10, 14, 182);
            style.visuals.widgets.noninteractive.bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 14);
            style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 32),
            );
            style.visuals.widgets.inactive.bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20);
            style.visuals.widgets.inactive.weak_bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10);
            style.visuals.widgets.active.bg_fill =
                egui::Color32::from_rgba_unmultiplied(140, 176, 255, 58);
            style.visuals.widgets.hovered.bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 28);
            style.visuals.window_stroke = egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38),
            );
        } else {
            style.visuals.panel_fill = egui::Color32::from_rgb(23, 25, 30);
            style.visuals.window_fill = egui::Color32::from_rgb(34, 37, 44);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(46, 50, 58);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(12, 13, 16);
            style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(34, 37, 44);
            style.visuals.widgets.noninteractive.bg_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(62, 68, 78));
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(44, 48, 56);
            style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(34, 37, 44);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(64, 88, 140);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(52, 58, 68);
            style.visuals.window_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(62, 68, 78));
        }

        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        ctx.set_style(style);
    }

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
                    glass_enabled: cfg!(target_os = "macos"),
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
                    theme_editor_demo: ThemeEditor::default(),
                    theme_editor_last_action: "No theme action yet.".into(),
                }
            }
            Err(error) => Self {
                registry: FeatureRegistry::empty(default_workspace_root()),
                selected_feature_id: None,
                search_query: String::new(),
                kind_filter: KindFilter::All,
                glass_enabled: cfg!(target_os = "macos"),
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
                theme_editor_demo: ThemeEditor::default(),
                theme_editor_last_action: "No theme action yet.".into(),
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

    pub(crate) fn feature_counts(&self) -> FeatureCounts {
        let mut counts = FeatureCounts {
            total: 0,
            ui: 0,
            logic: 0,
            workflow: 0,
        };
        for feature in self.registry.features() {
            counts.total += 1;
            if feature.manifest.id.starts_with("ui.") {
                counts.ui += 1;
            } else if feature.manifest.id.starts_with("logic.") {
                counts.logic += 1;
            } else if feature.manifest.id.starts_with("workflow.") {
                counts.workflow += 1;
            }
        }
        counts
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

    pub(crate) fn top_bar_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(18, 20, 24, 124)
        } else {
            egui::Color32::from_rgb(20, 22, 27)
        }
    }

    pub(crate) fn left_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(20, 22, 28, 104)
        } else {
            egui::Color32::from_rgb(24, 26, 32)
        }
    }

    pub(crate) fn right_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(22, 24, 30, 112)
        } else {
            egui::Color32::from_rgb(27, 29, 36)
        }
    }

    pub(crate) fn bottom_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(18, 20, 24, 116)
        } else {
            egui::Color32::from_rgb(21, 23, 29)
        }
    }

    pub(crate) fn central_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(10, 12, 16, 72)
        } else {
            egui::Color32::from_rgb(17, 19, 24)
        }
    }

    pub(crate) fn chrome_stroke(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 28)
        } else {
            egui::Color32::from_rgb(64, 70, 82)
        }
    }

    pub(crate) fn latest_activity_entry(&self) -> Option<&str> {
        self.activity_log.last().map(String::as_str)
    }

    pub(crate) fn test_output_state(&self) -> (&'static str, egui::Color32) {
        let trimmed = self.test_output.trim();
        let lowered = trimmed.to_lowercase();

        if trimmed == "No tests run yet." {
            ("idle", egui::Color32::from_rgb(188, 199, 220))
        } else if trimmed.contains("--- stderr ---")
            || lowered.contains("error")
            || lowered.contains("failed")
            || lowered.contains("panic")
        {
            ("error", egui::Color32::from_rgb(255, 133, 133))
        } else {
            ("ready", egui::Color32::from_rgb(126, 217, 140))
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
        Self::apply_visuals(ctx, self.glass_enabled);
        let style = ctx.style();
        let counts = self.feature_counts();
        let visible_count = self.feature_rows().len();
        let selected_summary = self
            .selected_feature()
            .map(|feature| {
                format!(
                    "{} · {}",
                    feature.manifest.id,
                    feature.manifest.status.to_string()
                )
            })
            .unwrap_or_else(|| "No feature selected".into());

        egui::TopBottomPanel::top("top_bar")
            .frame(glass_panel_frame(
                style.as_ref(),
                self.top_bar_fill(),
                self.chrome_stroke(),
            ))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    #[cfg(target_os = "macos")]
                    ui.add_space(84.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.heading("Feature Lab");
                        ui.small("Rust feature bench for isolated, testable, reusable crates.");
                        ui.separator();
                        ui.small(
                            egui::RichText::new(selected_summary.clone())
                                .monospace()
                                .color(egui::Color32::from_gray(185)),
                        );
                    });
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        toolbar_card(ui, |ui| {
                            ui.label("Search");
                            ui.add_sized(
                                [250.0, 28.0],
                                egui::TextEdit::singleline(&mut self.search_query),
                            );
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
                            pill(
                                ui,
                                format!("{visible_count} visible"),
                                egui::Color32::from_rgb(188, 199, 220),
                            );
                        });
                        toolbar_card(ui, |ui| {
                            pill(ui, format!("{} total", counts.total), self.chrome_stroke());
                            pill(
                                ui,
                                format!("{} ui", counts.ui),
                                egui::Color32::from_rgb(126, 188, 255),
                            );
                            pill(
                                ui,
                                format!("{} logic", counts.logic),
                                egui::Color32::from_rgb(255, 201, 110),
                            );
                            pill(
                                ui,
                                format!("{} workflows", counts.workflow),
                                egui::Color32::from_rgb(160, 150, 255),
                            );
                        });
                        toolbar_card(ui, |ui| {
                            #[cfg(target_os = "macos")]
                            ui.checkbox(&mut self.glass_enabled, "Glass");
                            if ui.button("Run Tests").clicked() {
                                self.run_selected_tests();
                            }
                            if ui.button("Open Feature Folder").clicked() {
                                self.open_selected_feature_folder();
                            }
                        });
                    });
                });
            });

        egui::SidePanel::left("feature_list")
            .resizable(true)
            .default_width(260.0)
            .frame(glass_panel_frame(
                style.as_ref(),
                self.left_panel_fill(),
                self.chrome_stroke(),
            ))
            .show(ctx, |ui| feature_list::show(ui, self));

        egui::SidePanel::right("feature_detail")
            .resizable(true)
            .default_width(320.0)
            .frame(glass_panel_frame(
                style.as_ref(),
                self.right_panel_fill(),
                self.chrome_stroke(),
            ))
            .show(ctx, |ui| feature_detail::show(ui, self));

        egui::TopBottomPanel::bottom("test_output")
            .resizable(true)
            .default_height(220.0)
            .frame(glass_panel_frame(
                style.as_ref(),
                self.bottom_panel_fill(),
                self.chrome_stroke(),
            ))
            .show(ctx, |ui| {
                let total_size = ui.available_size_before_wrap();
                let spacing = ui.spacing().item_spacing.x;
                let left_width = ((total_size.x - spacing) * 0.6).max(280.0);
                let right_width = (total_size.x - spacing - left_width).max(180.0);

                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(left_width, total_size.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| test_runner::show(ui, self),
                    );
                    ui.allocate_ui_with_layout(
                        egui::vec2(right_width, total_size.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| activity_log::show(ui, self),
                    );
                });
            });

        egui::TopBottomPanel::bottom("status_strip")
            .resizable(false)
            .exact_height(42.0)
            .frame(glass_panel_frame(
                style.as_ref(),
                self.bottom_panel_fill(),
                self.chrome_stroke(),
            ))
            .show(ctx, |ui| {
                let (test_state, test_state_color) = self.test_output_state();
                let visible_count = self.feature_rows().len();
                let selected_feature = self
                    .selected_feature_id
                    .as_deref()
                    .unwrap_or("No feature selected");
                let latest_event = self.latest_activity_entry().unwrap_or("No activity yet.");

                ui.horizontal_wrapped(|ui| {
                    status_pill(
                        ui,
                        format!("selected {selected_feature}"),
                        egui::Color32::from_rgb(188, 199, 220),
                    );
                    status_pill(
                        ui,
                        format!("filter {}", self.kind_filter.label()),
                        egui::Color32::from_rgb(126, 188, 255),
                    );
                    status_pill(
                        ui,
                        format!("{visible_count} visible"),
                        egui::Color32::from_rgb(255, 201, 110),
                    );
                    status_pill(ui, format!("tests {test_state}"), test_state_color);
                    ui.separator();
                    ui.small(
                        egui::RichText::new(format!("latest: {latest_event}"))
                            .monospace()
                            .color(egui::Color32::from_gray(185)),
                    );
                });
            });

        egui::CentralPanel::default()
            .frame(glass_central_frame(
                style.as_ref(),
                self.central_panel_fill(),
                self.chrome_stroke(),
            ))
            .show(ctx, |ui| demo_panel::show(ui, self));
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        #[cfg(target_os = "macos")]
        {
            if self.glass_enabled {
                egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
            } else {
                egui::Color32::from_rgb(14, 16, 20).to_normalized_gamma_f32()
            }
        }
        #[cfg(not(target_os = "macos"))]
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    }
}

fn glass_panel_frame(
    style: &egui::Style,
    fill: egui::Color32,
    stroke: egui::Color32,
) -> egui::Frame {
    egui::Frame::side_top_panel(style)
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
}

fn glass_central_frame(
    style: &egui::Style,
    fill: egui::Color32,
    stroke: egui::Color32,
) -> egui::Frame {
    egui::Frame::central_panel(style)
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
}

fn pill(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
    let text = text.into();
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            22,
        ))
        .stroke(egui::Stroke::new(1.0, color))
        .corner_radius(999.0)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.small(egui::RichText::new(text).color(color).strong());
        });
}

fn toolbar_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
        ))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| add_contents(ui));
        });
}

fn status_pill(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
    let text = text.into();
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            20,
        ))
        .stroke(egui::Stroke::new(1.0, color))
        .corner_radius(999.0)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.small(egui::RichText::new(text).color(color).strong().monospace());
        });
}
