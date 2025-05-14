use std::collections::HashSet;

use anyhow::{anyhow, Result};
use log::info;
use vulkanalia::vk::{self, EntryV1_0, ExtDebugUtilsExtension, HasBuilder};
use vulkanalia::window::{self as vk_window};
use vulkanalia::Entry;
use vulkanalia::Instance;
use winit::window::Window;

use crate::freyja::validation_layer::VALIDATION_LAYER;

use super::app_data::AppData;
use super::debug_callback::{self, debug_callback};
use super::portability_macos_version::PORTABILITY_MACOS_VERSION;
use super::validation_enabled::VALIDATION_ENABLED;

pub unsafe fn create_instance(
  window: &Window,
  entry: &Entry,
  data: &mut AppData,
) -> Result<Instance> {
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
