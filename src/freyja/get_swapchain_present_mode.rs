use vulkanalia::vk;

pub fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
  present_modes
    .iter()
    .cloned()
    .find(|m| *m == vk::PresentModeKHR::MAILBOX)
    .unwrap_or(vk::PresentModeKHR::FIFO)
}
