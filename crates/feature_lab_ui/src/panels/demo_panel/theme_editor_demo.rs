mod actions;
mod controls;
mod inspector;
mod preview;

use eframe::egui;

use crate::app::FeatureLabApp;

use self::actions::show_action_bar;
use self::controls::{ThemeEditorDraft, apply_draft_changes, show_controls};
use self::inspector::show_inspector;
use self::preview::show_preview;

pub(super) fn show_theme_editor_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut action_log = None;

    show_action_bar(ui, app, &mut action_log);
    ui.add_space(8.0);

    let mut draft = ThemeEditorDraft::from_app(app);

    ui.columns(3, |columns| {
        show_controls(&mut columns[0], &mut draft);
        show_preview(&mut columns[1], app);
        show_inspector(&mut columns[2], app);
    });

    apply_draft_changes(app, draft, &mut action_log);

    if let Some(message) = action_log {
        app.push_log(message);
    }
}
