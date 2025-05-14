use anyhow::Result;
use vulkanalia::{
  vk::{self, Handle, HasBuilder, KhrSwapchainExtension},
  Device, Instance,
};
use winit::window::Window;

use super::{
  app_data::AppData, get_swapchain_extent::get_swapchain_extent,
  get_swapchain_present_mode::get_swapchain_present_mode,
  get_swapchain_surface_format::get_swapchain_surface_format,
  queue_family_indices::QueueFamilyIndices, swapchain_support::SwapchainSupport,
};

pub unsafe fn create_swapchain(
  window: &Window,
  instance: &Instance,
  device: &Device,
  data: &mut AppData,
) -> Result<()> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
  let support = SwapchainSupport::get(instance, data, data.physical_device)?;
  let surface_format = get_swapchain_surface_format(&support.formats);
  let present_mode = get_swapchain_present_mode(&support.present_modes);
  let extent = get_swapchain_extent(window, support.capabilities);
  let mut image_count = support.capabilities.min_image_count + 1;

  if support.capabilities.max_image_count != 0 && image_count > support.capabilities.max_image_count
  {
    image_count = support.capabilities.max_image_count;
  }

  let mut queue_family_indices = vec![];
  let image_sharing_mode = if indices.graphics == indices.present {
    vk::SharingMode::EXCLUSIVE
  } else {
    queue_family_indices.push(indices.graphics);
    queue_family_indices.push(indices.present);
    vk::SharingMode::CONCURRENT
  };
  let info = vk::SwapchainCreateInfoKHR::builder()
    .surface(data.surface)
    .min_image_count(image_count)
    .image_format(surface_format.format)
    .image_color_space(surface_format.color_space)
    .image_extent(extent)
    .image_array_layers(1)
    .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
    .image_sharing_mode(image_sharing_mode)
    .queue_family_indices(&queue_family_indices)
    .pre_transform(support.capabilities.current_transform)
    .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
    .present_mode(present_mode)
    .clipped(true)
    .old_swapchain(vk::SwapchainKHR::null());

  data.swapchain = device.create_swapchain_khr(&info, None)?;

  Ok(())
}
