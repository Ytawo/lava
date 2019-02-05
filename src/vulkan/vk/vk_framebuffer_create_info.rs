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
use vulkan::vk::{VkFramebufferCreateFlags,RawVkFramebufferCreateFlags};
use vulkan::vk::{VkRenderPass,RawVkRenderPass};
use vulkan::vk::{VkImageView,RawVkImageView};

/// Wrapper for [VkFramebufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFramebufferCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkFramebufferCreateInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    pub flags: VkFramebufferCreateFlags,
    pub render_pass: &'a VkRenderPass,
    pub attachments: &'b [&'c VkImageView],
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkFramebufferCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkFramebufferCreateFlags,
    pub render_pass: RawVkRenderPass,
    pub attachment_count: u32,
    pub attachments: *mut RawVkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkFramebufferCreateInfo> for VkFramebufferCreateInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    fn vk_to_raw(src: &VkFramebufferCreateInfo, dst: &mut RawVkFramebufferCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FramebufferCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.render_pass = vk_to_raw_value(src.render_pass);
        dst.attachment_count = src.attachments.len() as u32;
        dst.attachments = new_ptr_vk_array_from_ref(src.attachments);
        dst.width = src.width;
        dst.height = src.height;
        dst.layers = src.layers;
    }
}

impl Default for VkFramebufferCreateInfo<'static, 'static, 'static> {
    fn default() -> VkFramebufferCreateInfo<'static, 'static, 'static> {
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

impl<'a, 'b, 'c> VkSetup for VkFramebufferCreateInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkFramebufferCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.attachments);
    }
}