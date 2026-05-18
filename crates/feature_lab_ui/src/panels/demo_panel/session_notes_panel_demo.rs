mod compose_mode;
mod edit_history;
mod edit_mode;
mod overview;

use eframe::egui;
use session_notes_panel::SessionNotesPanelMode;

use crate::app::FeatureLabApp;

use self::compose_mode::show_compose_mode;
use self::edit_mode::show_edit_mode;
use self::overview::show_overview;
use super::cards::empty_state_card;

pub(super) fn show_session_notes_panel_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    show_overview(ui, app);

    match app.session_notes_panel_demo.mode {
        SessionNotesPanelMode::Browse => {
            ui.add_space(12.0);
            empty_state_card(
                ui,
                "Start composing or edit the selected note to exercise the panel adapter.",
            );
        }
        SessionNotesPanelMode::Compose => show_compose_mode(ui, app),
        SessionNotesPanelMode::EditSelected => show_edit_mode(ui, app),
    }
}
