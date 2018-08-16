// Generated by `scripts/generate_vk.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_subpass_description_flags::*;
use vk::vk_pipeline_bind_point::*;
use vk::vk_attachment_reference::*;

#[repr(C)]
pub struct RawVkSubpassDescription {
    flags: RawVkSubpassDescriptionFlags,
    pipeline_bind_point: RawVkPipelineBindPoint,
    input_attachment_count: u32,
    input_attachments: *mut RawVkAttachmentReference,
    color_attachment_count: u32,
    color_attachments: *mut RawVkAttachmentReference,
    resolve_attachments: *mut RawVkAttachmentReference,
    depth_stencil_attachment: *mut RawVkAttachmentReference,
    preserve_attachment_count: u32,
    preserve_attachments: *mut u32,
}

#[derive(Debug, Clone)]
pub struct VkSubpassDescription<'a, 'b, 'c, 'd, 'e> {
    pub flags: VkSubpassDescriptionFlags,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub input_attachments: &'a [VkAttachmentReference],
    pub color_attachments: &'b [VkAttachmentReference],
    pub resolve_attachments: Option<&'c [VkAttachmentReference]>,
    pub depth_stencil_attachment: Option<&'d VkAttachmentReference>,
    pub preserve_attachments: &'e [usize],
}

impl<'a, 'b, 'c, 'd, 'e> VkWrappedType<RawVkSubpassDescription> for VkSubpassDescription<'a, 'b, 'c, 'd, 'e> {
    fn vk_to_raw(src: &VkSubpassDescription, dst: &mut RawVkSubpassDescription) {
        dst.flags = vk_to_raw_value(&src.flags);
        dst.pipeline_bind_point = vk_to_raw_value(&src.pipeline_bind_point);
        dst.input_attachment_count = src.input_attachments.len() as u32;
        dst.input_attachments = new_ptr_vk_array(src.input_attachments);
        dst.color_attachment_count = cmp::max(src.color_attachments.len(), get_array_option_len(src.resolve_attachments)) as u32;
        dst.color_attachments = new_ptr_vk_array(src.color_attachments);
        dst.resolve_attachments = new_ptr_vk_array_checked(src.resolve_attachments);
        dst.depth_stencil_attachment = new_ptr_vk_value_checked(src.depth_stencil_attachment);
        dst.preserve_attachment_count = src.preserve_attachments.len() as u32;
        dst.preserve_attachments = new_ptr_vk_array(src.preserve_attachments);
    }
}

impl Default for VkSubpassDescription<'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkSubpassDescription<'static, 'static, 'static, 'static, 'static> {
        VkSubpassDescription {
            flags: VkSubpassDescriptionFlags::default(),
            pipeline_bind_point: VkPipelineBindPoint::default(),
            input_attachments: &[],
            color_attachments: &[],
            resolve_attachments: None,
            depth_stencil_attachment: None,
            preserve_attachments: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e> VkSetup for VkSubpassDescription<'a, 'b, 'c, 'd, 'e> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassDescription {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.input_attachment_count as usize, self.input_attachments);
        free_vk_ptr_array(self.color_attachment_count as usize, self.color_attachments);
        free_vk_ptr_array(self.color_attachment_count as usize, self.resolve_attachments);
        free_vk_ptr(self.depth_stencil_attachment);
        free_ptr(self.preserve_attachments);
    }
}