use eframe::egui;
use feature_runner::{open_path_in_file_browser, run_feature_tests};

use super::FeatureLabApp;

impl FeatureLabApp {
    pub(crate) fn push_log(&mut self, message: impl Into<String>) {
        self.activity_log.push(message.into());
    }

    pub(crate) fn run_selected_tests(&mut self) {
        let Some(feature_id) = self.selected_feature_id().map(str::to_owned) else {
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
}
