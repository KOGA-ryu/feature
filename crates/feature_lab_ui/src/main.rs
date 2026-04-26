mod app;
#[cfg(target_os = "macos")]
mod macos_glass;
mod panels;

use app::FeatureLabApp;

fn main() -> eframe::Result<()> {
    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([1440.0, 920.0])
        .with_min_inner_size([1024.0, 720.0]);
    #[cfg(target_os = "macos")]
    {
        viewport = viewport
            .with_transparent(true)
            .with_fullsize_content_view(true)
            .with_titlebar_shown(false)
            .with_title_shown(false);
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Feature Lab",
        options,
        Box::new(|creation_context| {
            FeatureLabApp::configure_platform_visuals(&creation_context.egui_ctx);
            #[cfg(target_os = "macos")]
            if let Err(error) = macos_glass::configure_window(creation_context) {
                eprintln!("feature_lab_ui macOS glass setup skipped: {error}");
            }
            Ok(Box::new(FeatureLabApp::bootstrap()))
        }),
    )
}
