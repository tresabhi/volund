use anyhow::{anyhow, Ok, Result};
use vulkanalia::vk::{
  DeviceV1_0, ExtDebugUtilsExtension, InstanceV1_0, KhrSurfaceExtension, KhrSwapchainExtension,
};
use vulkanalia::window::{self as vk_window};
use vulkanalia::{
  loader::{LibloadingLoader, LIBRARY},
  Device, Entry, Instance,
};
use winit::window::Window;

use super::create_logical_device::create_logical_device;
use super::create_pipeline::create_pipeline;
use super::create_render_pass::{self, create_render_pass};
use super::create_swapchain::create_swapchain;
use super::create_swapchain_image_views::create_swapchain_image_views;
use super::pick_physical_device::pick_physical_device;
use super::validation_enabled::VALIDATION_ENABLED;
use super::{app_data::AppData, create_instance::create_instance};

#[derive(Clone, Debug)]
pub struct App {
  entry: Entry,
  instance: Instance,
  pub data: AppData,
  device: Device,
}

impl App {
  pub unsafe fn create(window: &Window) -> Result<Self> {
    let loader = LibloadingLoader::new(LIBRARY)?;
    let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
    let mut data = AppData::default();
    let instance = create_instance(window, &entry, &mut data)?;

    data.surface = vk_window::create_surface(&instance, &window, &window)?;
    pick_physical_device(&instance, &mut data)?;

    let device = create_logical_device(&entry, &instance, &mut data)?;

    create_swapchain(window, &instance, &device, &mut data)?;
    create_swapchain_image_views(&device, &mut data)?;
    create_render_pass(&instance, &device, &mut data)?;
    create_pipeline(&device, &mut data)?;

    Ok(Self {
      entry,
      instance,
      data,
      device,
    })
  }

  pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
    Ok(())
  }

  pub unsafe fn destroy(&mut self) {
    self
      .device
      .destroy_pipeline_layout(self.data.pipeline_layout, None);
    self.device.destroy_render_pass(self.data.render_pass, None);
    self
      .device
      .destroy_pipeline_layout(self.data.pipeline_layout, None);

    self
      .data
      .swapchain_image_views
      .iter()
      .for_each(|v| self.device.destroy_image_view(*v, None));
    self.device.destroy_swapchain_khr(self.data.swapchain, None);
    self.device.destroy_device(None);

    self.instance.destroy_surface_khr(self.data.surface, None);

    if VALIDATION_ENABLED {
      self
        .instance
        .destroy_debug_utils_messenger_ext(self.data.messenger, None);
    }

    self.instance.destroy_instance(None);
  }
}
