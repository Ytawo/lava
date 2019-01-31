// Generated by `scripts/generate.js`

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
use vk::vk_image_layout::*;
use vk::vk_image_aspect_flags::*;

#[derive(Debug, Clone)]
pub struct VkAttachmentReference2 {
    pub attachment: usize,
    pub layout: VkImageLayout,
    pub aspect_mask: VkImageAspectFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkAttachmentReference2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub attachment: u32,
    pub layout: RawVkImageLayout,
    pub aspect_mask: RawVkImageAspectFlags,
}

impl VkWrappedType<RawVkAttachmentReference2> for VkAttachmentReference2 {
    fn vk_to_raw(src: &VkAttachmentReference2, dst: &mut RawVkAttachmentReference2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::AttachmentReference2Khr);
        dst.next = ptr::null();
        dst.attachment = vk_to_raw_value(&src.attachment);
        dst.layout = vk_to_raw_value(&src.layout);
        dst.aspect_mask = vk_to_raw_value(&src.aspect_mask);
    }
}

impl VkRawType<VkAttachmentReference2> for RawVkAttachmentReference2 {
    fn vk_to_wrapped(src: &RawVkAttachmentReference2) -> VkAttachmentReference2 {
        VkAttachmentReference2 {
            attachment: u32::vk_to_wrapped(&src.attachment),
            layout: RawVkImageLayout::vk_to_wrapped(&src.layout),
            aspect_mask: RawVkImageAspectFlags::vk_to_wrapped(&src.aspect_mask),
        }
    }
}

impl Default for VkAttachmentReference2 {
    fn default() -> VkAttachmentReference2 {
        VkAttachmentReference2 {
            attachment: 0,
            layout: VkImageLayout::default(),
            aspect_mask: VkImageAspectFlags::default(),
        }
    }
}

impl VkSetup for VkAttachmentReference2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkAttachmentReference2 {
    fn vk_free(&mut self) {
        
    }
}