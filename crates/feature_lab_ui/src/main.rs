mod app;
mod panels;

use app::FeatureLabApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 920.0])
            .with_min_inner_size([1024.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Feature Lab",
        options,
        Box::new(|_creation_context| Ok(Box::new(FeatureLabApp::bootstrap()))),
    )
}
