//! This is where you write the app
use egui::*;
use log::{debug, info};
use eframe::egui;
use egui_extras;

/* Features: Eraser, eyedropper, twocolour choice (left/right click), paint bucket, undo/redo
 Colour palatte: Microsoft paint colour palatte + transparent colour
 Picture import/export to png
 Zoom: Ctrl + & Ctrl -
*/

const TOOL_BUTTON_SIZE: f32 = 30.0;
const CANVAS_LENGTH: usize = 16;

enum Tool {
    Pencil,
    Eraser,
    Eyedropper,
    PaintBucket
}

// Mspaint palatte + transparent: https://lospec.com/palette-list/mspaint-windows-7

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)] // All values are unsigned 32-bit integers, in ARGB format
pub enum Color {
    Transparent = 0x00000000,
    Black       = 0xFF000000,
    LightGrey   = 0xFF7F7F7F,
    DarkGrey    = 0xFFC3C3C3,
    White       = 0xFFFFFFFF,
    Maroon      = 0xFF880015,
    Red         = 0xFFED1C24,
    Brown       = 0xFFB97A57,
    Orange      = 0xFFFF7F27,
    Gold        = 0xFFFFC90E,
    Yellow      = 0xFFFFF200,
    Beige       = 0xFFEFE4B0,
    Lime        = 0xFFB5E61D,
    Green       = 0xFF22B14C,
    SkyBlue     = 0xFF99D9EA,
    SteelBlue   = 0xFF7092BE,
    Cyan        = 0xFF00A2E8,
    Indigo      = 0xFF3F48CC,
    Purple      = 0xFFA349A4,
    Pink        = 0xFFFFAEC9,
    Lavender    = 0xFFC8BFE7,
}

impl Color {
    fn to_color32(self) -> Color32 {
        // Bit manipulation fuckery that AI wrote

        let v = self as u32;

        let a = ((v >> 24) & 0xFF) as u8;
        let r = ((v >> 16) & 0xFF) as u8;
        let g = ((v >> 8)  & 0xFF) as u8;
        let b = (v & 0xFF) as u8;

        Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}

pub struct App {
    label: String,
    value: f32,
    eraser_image: ImageSource<'static>,
    eyedropper_image: ImageSource<'static>,
    paintbucket_image: ImageSource<'static>,
    pencil_image: ImageSource<'static>,
    selected_colour: Color,
    selected_tool: Tool,
    pixel_canvas: [Color; CANVAS_LENGTH * CANVAS_LENGTH]
}

impl App {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            eraser_image: include_image!("../assets/eraser_tool_button.png"),
            eyedropper_image: include_image!("../assets/eyedropper_tool_button.png"),
            paintbucket_image: include_image!("../assets/paintbucket_tool_button.png"),
            pencil_image: include_image!("../assets/pencil_tool_button.png"),
            label: "Hello World!".to_owned(),
            value: 2.7,
            selected_colour: Color::Black,
            selected_tool: Tool::Pencil,
            pixel_canvas: [Color::Transparent; CANVAS_LENGTH * CANVAS_LENGTH]
         }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        SidePanel::left("toolbar").show(ctx, |ui| {
            
            MenuBar::new().ui(ui, |ui| {

                // Light mode/dark mode switch
                widgets::global_theme_preference_switch(ui);
                
                ui.menu_button("File", |ui| {
                    if ui.button("Export as png").clicked() {

                    }
                    if ui.button("Import from png").clicked() {

                    }
                }); 

                if ui.button("Help").clicked() {

                }
        });

        ui.horizontal_wrapped(|ui| {
            let pencil_button = add_image_button(&self.pencil_image, ui);
            let eraser_button = add_image_button(&self.eraser_image, ui);
            let eyedropper_button = add_image_button(&self.eyedropper_image, ui);
            let paintbucket_button = add_image_button(&self.paintbucket_image, ui);

            if pencil_button.clicked() {
                self.selected_tool = Tool::Pencil;
            } else if eraser_button.clicked() {
                self.selected_tool = Tool::Eraser;
            } else if eyedropper_button.clicked() {
                self.selected_tool = Tool::Eyedropper;
            } else if paintbucket_button.clicked() {
                self.selected_tool = Tool::PaintBucket;
            }

            //ui.add(egui::Separator::default().horizontal().spacing(10.0));
            
            add_color_button(Color::Black, ui)

        })
    });

        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
            warn_if_debug_build(ui);
            });
        });
    }
}

fn add_image_button(image: &ImageSource, ui: &mut Ui) -> Response {
    let button_image = Image::new(image.clone())
    .fit_to_exact_size(Vec2::new(TOOL_BUTTON_SIZE, TOOL_BUTTON_SIZE));

    let button = Button::image(button_image);
    ui.add(button)
}

fn add_color_button(color: Color, ui: &mut Ui) -> Response {
    let button = Button::new("")
    .fill(color.to_color32())
    .min_size(Vec2::new(TOOL_BUTTON_SIZE, TOOL_BUTTON_SIZE));

    ui.add(button)
}


