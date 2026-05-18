mod selected_item;
mod stats;
mod toolbar;
mod visible_items;

use eframe::egui;

use crate::app::FeatureLabApp;

use self::selected_item::show_selected_item;
use self::stats::show_stats;
use self::toolbar::show_toolbar;
use self::visible_items::show_visible_items;

pub(super) fn show_quick_capture_inbox_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    show_toolbar(ui, app);
    show_stats(ui, app);
    show_visible_items(ui, app);
    show_selected_item(ui, app);
}
