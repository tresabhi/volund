// TODO: remove this
// #![allow(
//   dead_code,
//   unused_variables,
//   clippy::too_many_arguments,
//   clippy::unnecessary_wraps
// )]

use anyhow::{anyhow, Result};
use log::*;
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*; // TODO: update to latest prelude
use vulkanalia::vk::ExtExtendedDynamicState3Extension;
use vulkanalia::vk::{ExtDebugUtilsExtension, LayerProperties};
use vulkanalia::window::{self as vk_window, get_required_instance_extensions};
use vulkanalia::Version;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Icon;
use winit::window::{Window, WindowBuilder}; // TODO: update winit to the latest version

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName =
  vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

fn main() -> Result<()> {
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

  // App
  let mut app = unsafe { App::create(&window)? };
  event_loop.run(move |event, elwt| {
    match event {
      // Request a redraw when all events were processed.
      Event::AboutToWait => window.request_redraw(),
      Event::WindowEvent { event, .. } => match event {
        // Render a frame if our Vulkan app is not being destroyed.
        WindowEvent::RedrawRequested if !elwt.exiting() => unsafe { app.render(&window) }.unwrap(),
        // Destroy our Vulkan app.
        WindowEvent::CloseRequested => {
          elwt.exit();
          unsafe {
            app.destroy();
          }
        }
        _ => {}
      },
      _ => {}
    }
  })?;

  Ok(())
}

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
  let application_info = vk::ApplicationInfo::builder()
    .application_name(b"Volund\0")
    .application_version(vk::make_version(1, 0, 0))
    .engine_name(b"Apollo\0")
    .engine_version(vk::make_version(1, 0, 0))
    .api_version(vk::make_version(1, 1, 0));

  let mut extensions = vk_window::get_required_instance_extensions(window)
    .iter()
    .map(|e| e.as_ptr())
    .collect::<Vec<_>>();

  let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    info!("Enabling extensions for macOS portability");
    extensions.push(
      vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION
        .name
        .as_ptr(),
    );
    extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
    vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
  } else {
    vk::InstanceCreateFlags::empty()
  };

  let available_layers = entry
    .enumerate_instance_layer_properties()?
    .iter()
    .map(|l| l.layer_name)
    .collect::<HashSet<_>>();

  if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
    return Err(anyhow!("Validation layer requested but not supported."));
  }

  let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
  } else {
    Vec::new()
  };

  let info = vk::InstanceCreateInfo::builder()
    .application_info(&application_info)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .flags(flags);

  Ok(entry.create_instance(&info, None)?)
}

#[derive(Clone, Debug)]
struct App {
  entry: Entry,
  instance: Instance,
}

impl App {
  unsafe fn create(window: &Window) -> Result<Self> {
    let loader = LibloadingLoader::new(LIBRARY)?;
    let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
    let instance = create_instance(window, &entry)?;
    Ok(Self { entry, instance })
  }

  /// Renders a frame for our Vulkan app.
  unsafe fn render(&mut self, window: &Window) -> Result<()> {
    Ok(())
  }

  /// Destroys our Vulkan app.
  unsafe fn destroy(&mut self) {
    self.instance.destroy_instance(None);
  }
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
