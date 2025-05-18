use anyhow::{anyhow, Result};
use std::result::Result::Ok;
use vulkanalia::vk::{
  self, DeviceV1_0, ExtDebugUtilsExtension, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtension,
  KhrSwapchainExtension,
};
use vulkanalia::window::{self as vk_window};
use vulkanalia::{
  loader::{LibloadingLoader, LIBRARY},
  Device, Entry, Instance,
};
use winit::window::Window;

use super::create_command_buffers::create_command_buffers;
use super::create_command_pool::create_command_pool;
use super::create_framebuffers::create_framebuffers;
use super::create_logical_device::create_logical_device;
use super::create_pipeline::create_pipeline;
use super::create_render_pass::create_render_pass;
use super::create_swapchain::create_swapchain;
use super::create_swapchain_image_views::create_swapchain_image_views;
use super::create_sync_objects::create_sync_objects;
use super::max_frames_in_flight::MAX_FRAMES_IN_FLIGHT;
use super::pick_physical_device::pick_physical_device;
use super::validation_enabled::VALIDATION_ENABLED;
use super::{app_data::AppData, create_instance::create_instance};

#[derive(Clone, Debug)]
pub struct App {
  entry: Entry,
  instance: Instance,
  pub data: AppData,
  pub device: Device,
  frame: usize,
  pub resized: bool,
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
    create_framebuffers(&device, &mut data)?;
    create_command_pool(&instance, &device, &mut data)?;
    create_command_buffers(&device, &mut data)?;
    create_sync_objects(&device, &mut data)?;

    Ok(Self {
      entry,
      instance,
      data,
      device,
      frame: 0,
      resized: false,
    })
  }

  pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
    let in_flight_fence = self.data.in_flight_fences[self.frame];

    self
      .device
      .wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

    let result = self.device.acquire_next_image_khr(
      self.data.swapchain,
      u64::MAX,
      self.data.image_available_semaphores[self.frame],
      vk::Fence::null(),
    );

    let image_index = match result {
      Ok((image_index, _)) => image_index as usize,
      Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
      Err(e) => return Err(anyhow!(e)),
    };

    let image_in_flight = self.data.images_in_flight[image_index];

    if !image_in_flight.is_null() {
      self
        .device
        .wait_for_fences(&[image_in_flight], true, u64::MAX)?;
    }

    self.data.images_in_flight[image_index] = in_flight_fence;

    let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
    let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
    let command_buffers = &[self.data.command_buffers[image_index]];
    let signal_semaphores = &[self.data.render_finished_semaphores[self.frame]];
    let submit_info = vk::SubmitInfo::builder()
      .wait_semaphores(wait_semaphores)
      .wait_dst_stage_mask(wait_stages)
      .command_buffers(command_buffers)
      .signal_semaphores(signal_semaphores);

    self.device.reset_fences(&[in_flight_fence])?;

    self
      .device
      .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

    let swapchains = &[self.data.swapchain];
    let image_indices = &[image_index as u32];
    let present_info = vk::PresentInfoKHR::builder()
      .wait_semaphores(signal_semaphores)
      .swapchains(swapchains)
      .image_indices(image_indices);

    let result = self
      .device
      .queue_present_khr(self.data.present_queue, &present_info);
    let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
      || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

    if self.resized || changed {
      self.resized = false;
      self.recreate_swapchain(window)?;
    } else if let Err(e) = result {
      return Err(anyhow!(e));
    }

    self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

    Ok(())
  }

  unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
    self.device.device_wait_idle()?;
    self.destroy_swapchain();

    create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
    create_swapchain_image_views(&self.device, &mut self.data)?;
    create_render_pass(&self.instance, &self.device, &mut self.data)?;
    create_pipeline(&self.device, &mut self.data)?;
    create_framebuffers(&self.device, &mut self.data)?;
    create_command_buffers(&self.device, &mut self.data)?;

    self
      .data
      .images_in_flight
      .resize(self.data.swapchain_images.len(), vk::Fence::null());

    Ok(())
  }

  pub unsafe fn destroy(&mut self) {
    self.device.device_wait_idle().unwrap();

    self.destroy_swapchain();

    self
      .data
      .in_flight_fences
      .iter()
      .for_each(|f| self.device.destroy_fence(*f, None));
    self
      .data
      .render_finished_semaphores
      .iter()
      .for_each(|s| self.device.destroy_semaphore(*s, None));
    self
      .data
      .image_available_semaphores
      .iter()
      .for_each(|s| self.device.destroy_semaphore(*s, None));
    self
      .device
      .destroy_command_pool(self.data.command_pool, None);
    self.device.destroy_device(None);
    self.instance.destroy_surface_khr(self.data.surface, None);

    if VALIDATION_ENABLED {
      self
        .instance
        .destroy_debug_utils_messenger_ext(self.data.messenger, None);
    }

    self.instance.destroy_instance(None);
  }

  unsafe fn destroy_swapchain(&mut self) {
    self
      .device
      .free_command_buffers(self.data.command_pool, &self.data.command_buffers);
    self
      .data
      .framebuffers
      .iter()
      .for_each(|f| self.device.destroy_framebuffer(*f, None));
    self.device.destroy_pipeline(self.data.pipeline, None);
    self
      .device
      .destroy_pipeline_layout(self.data.pipeline_layout, None);
    self.device.destroy_render_pass(self.data.render_pass, None);
    self
      .data
      .swapchain_image_views
      .iter()
      .for_each(|v| self.device.destroy_image_view(*v, None));
    self.device.destroy_swapchain_khr(self.data.swapchain, None);
  }
}
