#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
            eframe::icon_data::from_png_bytes(&include_bytes!("../assets/paintbrush_icon.png")[..])
                .expect("Failed to load icon")
        ),
        ..Default::default()
    };
    eframe::run_native(
        "Pixel art editor",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(pixel_art_editor::App::new(cc)))
        }),
    )
}

