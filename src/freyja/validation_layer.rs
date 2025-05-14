use vulkanalia::vk;

pub const VALIDATION_LAYER: vk::ExtensionName =
  vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
