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
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::vk::{VkFramebufferAttachmentImageInfo,RawVkFramebufferAttachmentImageInfo};

/// Wrapper for [VkFramebufferAttachmentsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkFramebufferAttachmentsCreateInfo {
    pub attachment_image_infos: Vec<VkFramebufferAttachmentImageInfo>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkFramebufferAttachmentsCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub attachment_image_info_count: u32,
    pub attachment_image_infos: *mut RawVkFramebufferAttachmentImageInfo,
}

impl VkWrappedType<RawVkFramebufferAttachmentsCreateInfo> for VkFramebufferAttachmentsCreateInfo {
    fn vk_to_raw(src: &VkFramebufferAttachmentsCreateInfo, dst: &mut RawVkFramebufferAttachmentsCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FramebufferAttachmentsCreateInfo);
        dst.next = ptr::null_mut();
        dst.attachment_image_info_count = src.attachment_image_infos.len() as u32;
        dst.attachment_image_infos = new_ptr_vk_array(&src.attachment_image_infos);
    }
}

impl VkRawType<VkFramebufferAttachmentsCreateInfo> for RawVkFramebufferAttachmentsCreateInfo {
    fn vk_to_wrapped(src: &RawVkFramebufferAttachmentsCreateInfo) -> VkFramebufferAttachmentsCreateInfo {
        VkFramebufferAttachmentsCreateInfo {
            attachment_image_infos: new_vk_array(src.attachment_image_info_count, src.attachment_image_infos),
        }
    }
}

impl Default for VkFramebufferAttachmentsCreateInfo {
    fn default() -> VkFramebufferAttachmentsCreateInfo {
        VkFramebufferAttachmentsCreateInfo {
            attachment_image_infos: Vec::new(),
        }
    }
}

impl VkSetup for VkFramebufferAttachmentsCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkFramebufferAttachmentsCreateInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.attachment_image_info_count as usize, self.attachment_image_infos);
    }
}