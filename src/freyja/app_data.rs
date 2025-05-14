use vulkanalia::vk;

#[derive(Clone, Debug, Default)]
pub struct AppData {
  pub messenger: vk::DebugUtilsMessengerEXT,
  pub physical_device: vk::PhysicalDevice,
  pub graphics_queue: vk::Queue,
  pub surface: vk::SurfaceKHR,
  pub present_queue: vk::Queue,
  pub swapchain: vk::SwapchainKHR,
}
