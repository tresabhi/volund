use anyhow::Result;
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device,
};

use super::{app_data::AppData, create_shader_module::create_shader_module};

unsafe fn create_pipeline(device: &Device, data: &mut AppData) -> Result<()> {
  let vert = include_bytes!("../shaders/vert.spv");
  let frag = include_bytes!("../shaders/frag.spv");
  let vert_shader_module = create_shader_module(device, &vert[..])?;
  let frag_shader_module = create_shader_module(device, &frag[..])?;
  let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
    .stage(vk::ShaderStageFlags::VERTEX)
    .module(vert_shader_module)
    .name(b"main\0");
  let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
    .stage(vk::ShaderStageFlags::FRAGMENT)
    .module(frag_shader_module)
    .name(b"main\0");

  device.destroy_shader_module(vert_shader_module, None);
  device.destroy_shader_module(frag_shader_module, None);

  Ok(())
}
