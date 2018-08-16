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
use vk::vk_structure_type::*;
use vk::vk_framebuffer_create_flags::*;
use vk::vk_render_pass::*;
use vk::vk_image_view::*;

#[repr(C)]
pub struct RawVkFramebufferCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkFramebufferCreateFlags,
    render_pass: RawVkRenderPass,
    attachment_count: u32,
    attachments: *mut RawVkImageView,
    width: u32,
    height: u32,
    layers: u32,
}

#[derive(Debug, Clone)]
pub struct VkFramebufferCreateInfo<'a, 'b> {
    pub flags: VkFramebufferCreateFlags,
    pub render_pass: &'a VkRenderPass,
    pub attachments: &'b [VkImageView],
    pub width: usize,
    pub height: usize,
    pub layers: usize,
}

impl<'a, 'b> VkWrappedType<RawVkFramebufferCreateInfo> for VkFramebufferCreateInfo<'a, 'b> {
    fn vk_to_raw(src: &VkFramebufferCreateInfo, dst: &mut RawVkFramebufferCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FramebufferCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.render_pass = vk_to_raw_value(src.render_pass);
        dst.attachment_count = src.attachments.len() as u32;
        dst.attachments = new_ptr_vk_array(src.attachments);
        dst.width = vk_to_raw_value(&src.width);
        dst.height = vk_to_raw_value(&src.height);
        dst.layers = vk_to_raw_value(&src.layers);
    }
}

impl Default for VkFramebufferCreateInfo<'static, 'static> {
    fn default() -> VkFramebufferCreateInfo<'static, 'static> {
        VkFramebufferCreateInfo {
            flags: VkFramebufferCreateFlags::default(),
            render_pass: vk_null_ref(),
            attachments: &[],
            width: 0,
            height: 0,
            layers: 0,
        }
    }
}

impl<'a, 'b> VkSetup for VkFramebufferCreateInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkFramebufferCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.attachments);
    }
}