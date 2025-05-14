use anyhow::{anyhow, Ok, Result};
use vulkanalia::prelude::v1_0::*; // TODO: update to latest prelude

use crate::freyja::suitability_error::SuitabilityError;

use super::{app_data::AppData, queue_family_indices::QueueFamilyIndices};

pub unsafe fn check_physical_device(
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
