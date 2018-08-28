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
use vk::vk_image_format_properties::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageFormatProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image_format_properties: RawVkImageFormatProperties,
}

#[derive(Debug, Clone)]
pub struct VkImageFormatProperties2 {
    pub image_format_properties: VkImageFormatProperties,
}

impl VkRawType<VkImageFormatProperties2> for RawVkImageFormatProperties2 {
    fn vk_to_wrapped(src: &RawVkImageFormatProperties2) -> VkImageFormatProperties2 {
        VkImageFormatProperties2 {
            image_format_properties: RawVkImageFormatProperties::vk_to_wrapped(&src.image_format_properties),
        }
    }
}

impl VkWrappedType<RawVkImageFormatProperties2> for VkImageFormatProperties2 {
    fn vk_to_raw(src: &VkImageFormatProperties2, dst: &mut RawVkImageFormatProperties2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageFormatProperties2);
        dst.next = ptr::null();
        dst.image_format_properties = vk_to_raw_value(&src.image_format_properties);
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