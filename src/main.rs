// TODO: remove this
// #![allow(
//   dead_code,
//   unused_variables,
//   clippy::too_many_arguments,
//   clippy::unnecessary_wraps
// )]

use anyhow::{anyhow, Ok, Result};
use log::*;
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use thiserror::Error;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*; // TODO: update to latest prelude
use vulkanalia::vk::{DebugUtilsMessageSeverityFlagsEXT, ExtExtendedDynamicState3Extension};
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

unsafe fn create_logical_device(
  entry: &Entry,
  instance: &Instance,
  data: &mut AppData,
) -> Result<Device> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
  let queue_priorities = &[1.0];
  let queue_info = vk::DeviceQueueCreateInfo::builder()
    .queue_family_index(indices.graphics)
    .queue_priorities(queue_priorities);
  let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
  } else {
    vec![]
  };
  let mut extensions = vec![];

  if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
  }

  let features = vk::PhysicalDeviceFeatures::builder();
  let queue_infos = &[queue_info];
  let info = vk::DeviceCreateInfo::builder()
    .queue_create_infos(queue_infos)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .enabled_features(&features);
  let device = instance.create_device(data.physical_device, &info, None)?;

  data.graphics_queue = device.get_device_queue(indices.graphics, 0);

  Ok(device)
}

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
  graphics: u32,
}

impl QueueFamilyIndices {
  unsafe fn get(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
  ) -> Result<Self> {
    let properties = instance.get_physical_device_queue_family_properties(physical_device);
    let graphics = properties
      .iter()
      .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
      .map(|i| i as u32);

    if let Some(graphics) = graphics {
      Ok(Self { graphics })
    } else {
      Err(anyhow!(SuitabilityError(
        "Missing required queue families."
      )))
    }
  }
}

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
  for physical_device in instance.enumerate_physical_devices()? {
    let properties = instance.get_physical_device_properties(physical_device);

    if let Err(error) = check_physical_device(instance, data, physical_device) {
      // reconsider this
      warn!(
        "Skipping physical device (`{}`): {}",
        properties.device_name, error
      );
    } else {
      info!("Selected physical device (`{}`).", properties.device_name);

      data.physical_device = physical_device;

      return Ok(());
    }
  }

  Err(anyhow!("Failed to find suitable physical device."))
}

unsafe fn check_physical_device(
  instance: &Instance,
  data: &AppData,
  physical_device: vk::PhysicalDevice,
) -> Result<()> {
  let properties = instance.get_physical_device_properties(physical_device);
  if properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
    // TODO: reconsider this
    return Err(anyhow!(SuitabilityError(
      "Only discrete GPUs are supported."
    )));
  }

  let features = instance.get_physical_device_features(physical_device);
  if features.geometry_shader != vk::TRUE {
    return Err(anyhow!(SuitabilityError(
      "Missing geometry shader support."
    )));
  }

  QueueFamilyIndices::get(instance, data, physical_device)?;

  Ok(())
}

unsafe fn create_instance(window: &Window, entry: &Entry, data: &mut AppData) -> Result<Instance> {
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

  // TODO: merge these two if statements
  if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
    return Err(anyhow!("Validation layer requested but not supported."));
  }

  if VALIDATION_ENABLED {
    extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
  }

  let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
  } else {
    Vec::new()
  };
  let mut info = vk::InstanceCreateInfo::builder()
    .application_info(&application_info)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .flags(flags);
  let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
    .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
    .message_type(
      vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
    )
    .user_callback(Some(debug_callback));

  if VALIDATION_ENABLED {
    info = info.push_next(&mut debug_info);
  }

  let instance = entry.create_instance(&info, None)?;

  if VALIDATION_ENABLED {
    data.messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
  }

  Ok(instance)
}

extern "system" fn debug_callback(
  severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  type_: vk::DebugUtilsMessageTypeFlagsEXT,
  data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _: *mut c_void,
) -> vk::Bool32 {
  let data = unsafe { *data };
  let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

  if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
    error!("({:?}) {}", type_, message);
  } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
    warn!("({:?}) {}", type_, message);
  } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
    debug!("({:?}) {}", type_, message);
  } else {
    trace!("({:?}) {}", type_, message);
  }

  vk::FALSE
}

#[derive(Clone, Debug)]
struct App {
  entry: Entry,
  instance: Instance,
  data: AppData,
  device: Device,
}

impl App {
  unsafe fn create(window: &Window) -> Result<Self> {
    let loader = LibloadingLoader::new(LIBRARY)?;
    let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
    let mut data = AppData::default();
    let instance = create_instance(window, &entry, &mut data)?;

    pick_physical_device(&instance, &mut data)?;

    let device = create_logical_device(&entry, &instance, &mut data)?;

    Ok(Self {
      entry,
      instance,
      data,
      device,
    })
  }

  /// Renders a frame for our Vulkan app.
  unsafe fn render(&mut self, window: &Window) -> Result<()> {
    Ok(())
  }

  /// Destroys our Vulkan app.
  unsafe fn destroy(&mut self) {
    if VALIDATION_ENABLED {
      self
        .instance
        .destroy_debug_utils_messenger_ext(self.data.messenger, None);
    }

    self.instance.destroy_instance(None);
    self.device.destroy_device(None);
  }
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {
  messenger: vk::DebugUtilsMessengerEXT,
  physical_device: vk::PhysicalDevice,
  graphics_queue: vk::Queue,
}
