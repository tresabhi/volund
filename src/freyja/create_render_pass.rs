use anyhow::Result;
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device, Instance,
};

use super::app_data::AppData;

pub unsafe fn create_render_pass(
  instance: &Instance,
  device: &Device,
  data: &mut AppData,
) -> Result<()> {
  let color_attachment = vk::AttachmentDescription::builder()
    .format(data.swapchain_format)
    .samples(vk::SampleCountFlags::_1)
    .load_op(vk::AttachmentLoadOp::CLEAR)
    .store_op(vk::AttachmentStoreOp::STORE)
    .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
    .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
    .initial_layout(vk::ImageLayout::UNDEFINED)
    .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
  let color_attachment_ref = vk::AttachmentReference::builder()
    .attachment(0)
    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
  let color_attachments = &[color_attachment_ref];

  let subpass = vk::SubpassDescription::builder()
    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
    .color_attachments(color_attachments);
  let attachments = &[color_attachment];
  let subpasses = &[subpass];
  let info = vk::RenderPassCreateInfo::builder()
    .attachments(attachments)
    .subpasses(subpasses);

  data.render_pass = device.create_render_pass(&info, None)?;

  Ok(())
}
