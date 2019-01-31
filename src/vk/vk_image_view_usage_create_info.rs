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
use vk::vk_image_usage_flags::*;

#[derive(Debug, Clone)]
pub struct VkImageViewUsageCreateInfo {
    pub usage: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageViewUsageCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub usage: RawVkImageUsageFlags,
}

impl VkWrappedType<RawVkImageViewUsageCreateInfo> for VkImageViewUsageCreateInfo {
    fn vk_to_raw(src: &VkImageViewUsageCreateInfo, dst: &mut RawVkImageViewUsageCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageViewUsageCreateInfo);
        dst.next = ptr::null();
        dst.usage = vk_to_raw_value(&src.usage);
    }
}

impl VkRawType<VkImageViewUsageCreateInfo> for RawVkImageViewUsageCreateInfo {
    fn vk_to_wrapped(src: &RawVkImageViewUsageCreateInfo) -> VkImageViewUsageCreateInfo {
        VkImageViewUsageCreateInfo {
            usage: RawVkImageUsageFlags::vk_to_wrapped(&src.usage),
        }
    }
}

impl Default for VkImageViewUsageCreateInfo {
    fn default() -> VkImageViewUsageCreateInfo {
        VkImageViewUsageCreateInfo {
            usage: VkImageUsageFlags::default(),
        }
    }
}

impl VkSetup for VkImageViewUsageCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImageViewUsageCreateInfo {
    fn vk_free(&mut self) {
        
    }
}