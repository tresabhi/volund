use anyhow::Result;
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device,
};

use super::app_data::AppData;

pub unsafe fn create_swapchain_image_views(device: &Device, data: &mut AppData) -> Result<()> {
  data.swapchain_image_views = data
    .swapchain_images
    .iter()
    .map(|i| {
      let components = vk::ComponentMapping::builder()
        .r(vk::ComponentSwizzle::IDENTITY)
        .g(vk::ComponentSwizzle::IDENTITY)
        .b(vk::ComponentSwizzle::IDENTITY)
        .a(vk::ComponentSwizzle::IDENTITY);

      let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);

      let info = vk::ImageViewCreateInfo::builder()
        .image(*i)
        .view_type(vk::ImageViewType::_2D)
        .format(data.swapchain_format)
        .components(components)
        .subresource_range(subresource_range);

      device.create_image_view(&info, None)
    })
    .collect::<Result<Vec<_>, _>>()?;

  Ok(())
}
