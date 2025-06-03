use eframe::{egui, run_native, App, NativeOptions, Result};
use egui::{CentralPanel, Color32, Context, CornerRadius, Frame, Margin, ViewportBuilder};
use egui_extras::install_image_loaders;
use env_logger::fmt::style::Color;

fn main() -> Result {
  env_logger::init();

  let options = NativeOptions {
    viewport: ViewportBuilder::default(),
    ..Default::default()
  };

  run_native(
    "Volund",
    options,
    Box::new(|context| {
      install_image_loaders(&context.egui_ctx);
      Ok(Box::<MyApp>::default())
    }),
  )
}

struct MyApp {
  name: String,
  age: u32,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: "Arthur".to_owned(),
      age: 42,
    }
  }
}

impl App for MyApp {
  fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    CentralPanel::default().show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.label("Item 1");
        ui.button("Button 2");
      });
    });
  }
}
