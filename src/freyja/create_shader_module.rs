use anyhow::Result;
use vulkanalia::{
  bytecode::Bytecode,
  vk::{self, DeviceV1_0, HasBuilder},
  Device,
};

pub unsafe fn create_shader_module(device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
  let bytecode = Bytecode::new(bytecode).unwrap();
  let info = vk::ShaderModuleCreateInfo::builder().code(bytecode.code());

  Ok(device.create_shader_module(&info, None)?)
}
