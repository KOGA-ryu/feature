use eframe::egui;
use theme_editor::ThemeMode;

use crate::app::FeatureLabApp;

#[derive(Clone)]
pub(super) struct ThemeEditorDraft {
    mode: ThemeMode,
    accent: String,
    background: String,
    foreground: String,
    contrast: u8,
    translucent_sidebar: bool,
    ui_font: String,
    code_font: String,
}

impl ThemeEditorDraft {
    pub(super) fn from_app(app: &FeatureLabApp) -> Self {
        let config = app.theme_editor_demo.config();
        Self {
            mode: config.mode,
            accent: config.accent.clone(),
            background: config.background.clone(),
            foreground: config.foreground.clone(),
            contrast: config.contrast,
            translucent_sidebar: config.translucent_sidebar,
            ui_font: config.ui_font.clone(),
            code_font: config.code_font.clone(),
        }
    }
}

pub(super) fn show_controls(ui: &mut egui::Ui, draft: &mut ThemeEditorDraft) {
    ui.group(|ui| {
        ui.strong("Controls");
        egui::ComboBox::from_label("Mode")
            .selected_text(draft.mode.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut draft.mode,
                    ThemeMode::Light,
                    ThemeMode::Light.to_string(),
                );
                ui.selectable_value(
                    &mut draft.mode,
                    ThemeMode::Dark,
                    ThemeMode::Dark.to_string(),
                );
                ui.selectable_value(
                    &mut draft.mode,
                    ThemeMode::System,
                    ThemeMode::System.to_string(),
                );
            });
        ui.label("Accent");
        ui.text_edit_singleline(&mut draft.accent);
        ui.label("Background");
        ui.text_edit_singleline(&mut draft.background);
        ui.label("Foreground");
        ui.text_edit_singleline(&mut draft.foreground);
        ui.add(egui::Slider::new(&mut draft.contrast, 0..=100).text("Contrast"));
        ui.checkbox(&mut draft.translucent_sidebar, "Translucent sidebar");
        ui.label("UI font");
        ui.text_edit_singleline(&mut draft.ui_font);
        ui.label("Code font");
        ui.text_edit_singleline(&mut draft.code_font);
    });
}

pub(super) fn apply_draft_changes(
    app: &mut FeatureLabApp,
    draft: ThemeEditorDraft,
    action_log: &mut Option<String>,
) {
    let previous_mode = app.theme_editor_demo.config().mode;
    if draft.mode != previous_mode {
        app.theme_editor_demo.set_mode(draft.mode);
        app.theme_editor_last_action = format!("Mode set to {}.", draft.mode);
        *action_log = Some(format!("Theme editor mode set to {}.", draft.mode));
    }
    if draft.accent != app.theme_editor_demo.config().accent {
        app.theme_editor_demo.set_accent(draft.accent);
        app.theme_editor_last_action = "Accent updated.".into();
        *action_log = Some("Theme editor updated accent.".into());
    }
    if draft.background != app.theme_editor_demo.config().background {
        app.theme_editor_demo.set_background(draft.background);
        app.theme_editor_last_action = "Background updated.".into();
        *action_log = Some("Theme editor updated background.".into());
    }
    if draft.foreground != app.theme_editor_demo.config().foreground {
        app.theme_editor_demo.set_foreground(draft.foreground);
        app.theme_editor_last_action = "Foreground updated.".into();
        *action_log = Some("Theme editor updated foreground.".into());
    }
    if draft.contrast != app.theme_editor_demo.config().contrast {
        app.theme_editor_demo.set_contrast(draft.contrast);
        app.theme_editor_last_action = format!("Contrast set to {}.", draft.contrast);
        *action_log = Some(format!("Theme editor contrast set to {}.", draft.contrast));
    }
    if draft.translucent_sidebar != app.theme_editor_demo.config().translucent_sidebar {
        app.theme_editor_demo
            .set_translucent_sidebar(draft.translucent_sidebar);
        app.theme_editor_last_action = if draft.translucent_sidebar {
            "Enabled translucent sidebar.".into()
        } else {
            "Disabled translucent sidebar.".into()
        };
        *action_log = Some("Theme editor toggled translucent sidebar.".into());
    }
    if draft.ui_font != app.theme_editor_demo.config().ui_font {
        app.theme_editor_demo.set_ui_font(draft.ui_font);
        app.theme_editor_last_action = "UI font updated.".into();
        *action_log = Some("Theme editor updated UI font.".into());
    }
    if draft.code_font != app.theme_editor_demo.config().code_font {
        app.theme_editor_demo.set_code_font(draft.code_font);
        app.theme_editor_last_action = "Code font updated.".into();
        *action_log = Some("Theme editor updated code font.".into());
    }
}
