use vulkanalia::vk;

pub fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
  formats
    .iter()
    .cloned()
    .find(|f| {
      f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
    })
    .unwrap_or_else(|| formats[0])
}
