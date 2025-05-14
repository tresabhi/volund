use std::collections::HashSet;

use anyhow::{anyhow, Result};
use vulkanalia::{
  vk::{self, InstanceV1_0},
  Instance,
};

use crate::freyja::suitability_error::SuitabilityError;

use super::device_extensions::DEVICE_EXTENSIONS;

pub unsafe fn check_physical_device_extensions(
  instance: &Instance,
  physical_device: vk::PhysicalDevice,
) -> Result<()> {
  let extensions = instance
    .enumerate_device_extension_properties(physical_device, None)?
    .iter()
    .map(|e| e.extension_name)
    .collect::<HashSet<_>>();

  if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
    Ok(())
  } else {
    Err(anyhow!(SuitabilityError(
      "Missing required device extensions."
    )))
  }
}
