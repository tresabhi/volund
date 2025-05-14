use vulkanalia::vk;

#[derive(Clone, Debug, Default)]
pub struct AppData {
  pub messenger: vk::DebugUtilsMessengerEXT,
  pub physical_device: vk::PhysicalDevice,
  pub graphics_queue: vk::Queue,
  pub surface: vk::SurfaceKHR,
  pub present_queue: vk::Queue,
  pub swapchain: vk::SwapchainKHR,
  pub swapchain_images: Vec<vk::Image>,
  pub swapchain_format: vk::Format,
  pub swapchain_extent: vk::Extent2D,
  pub swapchain_image_views: Vec<vk::ImageView>,
  pub render_pass: vk::RenderPass,
  pub pipeline_layout: vk::PipelineLayout,
  pub pipeline: vk::Pipeline,
}
