use vulkanalia::vk::{self, HasBuilder};
use winit::window::Window;

pub fn get_swapchain_extent(
  window: &Window,
  capabilities: vk::SurfaceCapabilitiesKHR,
) -> vk::Extent2D {
  if capabilities.current_extent.width == u32::MAX {
    vk::Extent2D::builder()
      .width(window.inner_size().width.clamp(
        capabilities.min_image_extent.width,
        capabilities.max_image_extent.width,
      ))
      .height(window.inner_size().height.clamp(
        capabilities.min_image_extent.height,
        capabilities.max_image_extent.height,
      ))
      .build()
  } else {
    capabilities.current_extent
  }
}
