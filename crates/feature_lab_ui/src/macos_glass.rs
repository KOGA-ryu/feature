use eframe::CreationContext;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSColor, NSView, NSWindow};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

pub fn configure_window(creation_context: &CreationContext<'_>) -> Result<(), String> {
    let _main_thread = MainThreadMarker::new()
        .ok_or_else(|| "AppKit glass setup must run on the main thread".to_string())?;
    let window_handle = creation_context
        .window_handle()
        .map_err(|error| format!("missing native window handle: {error}"))?;
    let ns_view = match window_handle.as_raw() {
        RawWindowHandle::AppKit(handle) => {
            // SAFETY: eframe hands out a live AppKit window handle for this window.
            unsafe { Retained::<NSView>::retain(handle.ns_view.as_ptr().cast()) }
                .ok_or_else(|| "failed to retain native AppKit content view".to_string())?
        }
        other => {
            return Err(format!(
                "feature_lab_ui expected an AppKit handle on macOS, got {other:?}"
            ));
        }
    };

    let window = ns_view
        .window()
        .ok_or_else(|| "AppKit content view is not installed in a window yet".to_string())?;

    apply_window_glass_flags(&window);
    Ok(())
}

fn apply_window_glass_flags(window: &NSWindow) {
    let clear = NSColor::clearColor();
    window.setOpaque(false);
    window.setBackgroundColor(Some(&clear));
    window.setTitlebarAppearsTransparent(true);
}
