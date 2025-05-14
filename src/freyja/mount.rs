use anyhow::Result;
use winit::event::{Event, WindowEvent};
use winit::window::Icon;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

use super::app::App;

pub fn mount() -> Result<()> {
  pretty_env_logger::init();

  // Window
  let event_loop = EventLoop::new()?;
  let window = WindowBuilder::new()
    .with_title("Volund")
    .with_inner_size(LogicalSize::new(1024, 768))
    .build(&event_loop)?;

  let icon = {
    let image = image::open("src/icon.png")
      .expect("Failed to open icon")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
  };
  window.set_window_icon(Some(icon));

  let mut app = unsafe { App::create(&window)? };
  event_loop.run(move |event, elwt| match event {
    Event::AboutToWait => window.request_redraw(),
    Event::WindowEvent { event, .. } => match event {
      WindowEvent::RedrawRequested if !elwt.exiting() => unsafe { app.render(&window) }.unwrap(),
      WindowEvent::CloseRequested => {
        elwt.exit();
        unsafe {
          app.destroy();
        }
      }
      _ => {}
    },
    _ => {}
  })?;

  Ok(())
}
