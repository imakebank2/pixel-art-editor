//! This is where you write the app
//! 

/* Features: Eraser, eyedropper, twocolour choice (left/right click), paint bucket, undo/redo
 Colour palatte: Microsoft paint colour palatte + transparent colour
 Picture import/export to png
*/

// Canvas size is 8x8, 16x16 or 32x32 pixels.
enum CanvasSize {
    S8 = 8,
    S16 = 16,
    S32 = 32
}

pub struct App {
    label: String,
    value: f32,
    canvas_size: CanvasSize
}

impl App {
    // Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            // Default values: 
            label: "Hello World!".to_owned(),
            value: 2.7,
            canvas_size: CanvasSize::S16
         }

    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let scaling_factor = ctx.pixels_per_point();

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {

            // Light mode/dark mode switch
            egui::widgets::global_theme_preference_switch(ui);
            
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            
            // ui.add_space(16.0);
                
            });
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

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
            });
        });
    }
}