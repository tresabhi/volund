use anyhow::Result;
use winit::event::{Event, WindowEvent};
use winit::window::Icon;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

use super::app::App;

pub fn mount() -> Result<()> {
  pretty_env_logger::init();

  let event_loop = EventLoop::new()?;
  let window = WindowBuilder::new()
    .with_title("Vulkan Tutorial (Rust)")
    .with_inner_size(LogicalSize::new(1024, 768))
    .build(&event_loop)?;
  let mut app = unsafe { App::create(&window)? };
  let mut minimized = false;

  event_loop.run(move |event, elwt| match event {
    Event::AboutToWait => window.request_redraw(),

    Event::WindowEvent { event, .. } => match event {
      WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
        unsafe { app.render(&window) }.unwrap();
      }

      WindowEvent::Resized(size) => {
        if size.width == 0 || size.height == 0 {
          minimized = true;
        } else {
          minimized = false;
          app.resized = true;
        }
      }

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
