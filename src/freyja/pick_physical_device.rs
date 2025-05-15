use anyhow::{anyhow, Result};
use log::{info, warn};
use vulkanalia::{vk::InstanceV1_0, Instance};

use super::{app_data::AppData, check_physical_device::check_physical_device};

pub unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
  for physical_device in instance.enumerate_physical_devices()? {
    let properties = instance.get_physical_device_properties(physical_device);

    if let Err(error) = check_physical_device(instance, data, physical_device) {
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
