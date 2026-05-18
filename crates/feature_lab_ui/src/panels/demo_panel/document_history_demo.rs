mod revisions;
mod stats;
mod toolbar;
mod working_document;

use eframe::egui;

use crate::app::FeatureLabApp;

use self::revisions::show_revisions;
use self::stats::show_stats;
use self::toolbar::show_toolbar;
use self::working_document::show_working_document;

pub(super) fn show_document_history_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    show_toolbar(ui, app);
    show_stats(ui, app);
    show_working_document(ui, app);
    show_revisions(ui, app);
}
