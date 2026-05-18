mod selected_note;
mod stats;
mod toolbar;
mod visible_notes;

use eframe::egui;

use crate::app::FeatureLabApp;

use self::selected_note::show_selected_note;
use self::stats::show_stats;
use self::toolbar::show_toolbar;
use self::visible_notes::show_visible_notes;

pub(super) fn show_session_notes_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    show_toolbar(ui, app);
    show_stats(ui, app);
    show_visible_notes(ui, app);
    show_selected_note(ui, app);
}
