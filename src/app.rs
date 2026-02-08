//! This is where you write the app
use log::{debug, info};
use eframe::egui;
use egui_extras;

/* Features: Eraser, eyedropper, twocolour choice (left/right click), paint bucket, undo/redo
 Colour palatte: Microsoft paint colour palatte + transparent colour
 Picture import/export to png
 Zoom: Ctrl + & Ctrl -
*/

const CANVAS_SIZE: i8 = 16;

pub struct App {
    label: String,
    value: f32,
    eraser_image: egui::ImageSource<'static>,
    eyedropper_image: egui::ImageSource<'static>,
    paintbucket_image: egui::ImageSource<'static>,
    pencil_image: egui::ImageSource<'static>
}

impl App {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            eraser_image: egui::include_image!("../assets/eraser_tool_button.png"),
            eyedropper_image: egui::include_image!("../assets/eyedropper_tool_button.png"),
            paintbucket_image: egui::include_image!("../assets/paintbucket_tool_button.png"),
            pencil_image: egui::include_image!("../assets/pencil_tool_button.png"),
            label: "Hello World!".to_owned(),
            value: 2.7
         }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("toolbar").show(ctx, |ui| {
            
                egui::MenuBar::new().ui(ui, |ui| {

            // Light mode/dark mode switch
            egui::widgets::global_theme_preference_switch(ui);
            
            ui.menu_button("File", |ui| {
                if ui.button("Export as png").clicked() {

                }
                if ui.button("Import from png").clicked() {

                }
            });   

            if ui.button("Help").clicked() {

            }
        });
        ui.label("tools here");
    });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
            });
        });
    }
}