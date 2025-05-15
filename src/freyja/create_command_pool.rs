use anyhow::Result;
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device, Instance,
};

use super::{app_data::AppData, queue_family_indices::QueueFamilyIndices};

pub unsafe fn create_command_pool(
  instance: &Instance,
  device: &Device,
  data: &mut AppData,
) -> Result<()> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

  let info = vk::CommandPoolCreateInfo::builder().queue_family_index(indices.graphics);

  data.command_pool = device.create_command_pool(&info, None)?;

  Ok(())
}
