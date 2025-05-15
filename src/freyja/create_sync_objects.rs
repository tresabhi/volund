use anyhow::Result;
use vulkanalia::{
  vk::{self, HasBuilder},
  Device,
};

use super::app_data::AppData;

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
  let semaphore_info = vk::SemaphoreCreateInfo::builder();

  Ok(())
}
