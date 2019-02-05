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
use vulkan::vk::{VkImageFormatProperties,RawVkImageFormatProperties};

/// Wrapper for [VkImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageFormatProperties2.html).
#[derive(Debug, Clone)]
pub struct VkImageFormatProperties2 {
    pub image_format_properties: VkImageFormatProperties,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageFormatProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image_format_properties: RawVkImageFormatProperties,
}

impl VkWrappedType<RawVkImageFormatProperties2> for VkImageFormatProperties2 {
    fn vk_to_raw(src: &VkImageFormatProperties2, dst: &mut RawVkImageFormatProperties2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageFormatProperties2);
        dst.next = ptr::null();
        dst.image_format_properties = vk_to_raw_value(&src.image_format_properties);
    }
}

impl VkRawType<VkImageFormatProperties2> for RawVkImageFormatProperties2 {
    fn vk_to_wrapped(src: &RawVkImageFormatProperties2) -> VkImageFormatProperties2 {
        VkImageFormatProperties2 {
            image_format_properties: RawVkImageFormatProperties::vk_to_wrapped(&src.image_format_properties),
        }
    }
}

impl Default for VkImageFormatProperties2 {
    fn default() -> VkImageFormatProperties2 {
        VkImageFormatProperties2 {
            image_format_properties: VkImageFormatProperties::default(),
        }
    }
}

impl VkSetup for VkImageFormatProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.image_format_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkImageFormatProperties2 {
    fn vk_free(&mut self) {
        RawVkImageFormatProperties::vk_free(&mut self.image_format_properties);
    }
}