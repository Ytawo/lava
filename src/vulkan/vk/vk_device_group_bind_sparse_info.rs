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

/// Wrapper for [VkDeviceGroupBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceGroupBindSparseInfo.html).
#[derive(Debug, Clone)]
pub struct VkDeviceGroupBindSparseInfo {
    pub resource_device_index: usize,
    pub memory_device_index: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupBindSparseInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}

impl VkWrappedType<RawVkDeviceGroupBindSparseInfo> for VkDeviceGroupBindSparseInfo {
    fn vk_to_raw(src: &VkDeviceGroupBindSparseInfo, dst: &mut RawVkDeviceGroupBindSparseInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupBindSparseInfo);
        dst.next = ptr::null();
        dst.resource_device_index = vk_to_raw_value(&src.resource_device_index);
        dst.memory_device_index = vk_to_raw_value(&src.memory_device_index);
    }
}

impl VkRawType<VkDeviceGroupBindSparseInfo> for RawVkDeviceGroupBindSparseInfo {
    fn vk_to_wrapped(src: &RawVkDeviceGroupBindSparseInfo) -> VkDeviceGroupBindSparseInfo {
        VkDeviceGroupBindSparseInfo {
            resource_device_index: u32::vk_to_wrapped(&src.resource_device_index),
            memory_device_index: u32::vk_to_wrapped(&src.memory_device_index),
        }
    }
}

impl Default for VkDeviceGroupBindSparseInfo {
    fn default() -> VkDeviceGroupBindSparseInfo {
        VkDeviceGroupBindSparseInfo {
            resource_device_index: 0,
            memory_device_index: 0,
        }
    }
}

impl VkSetup for VkDeviceGroupBindSparseInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDeviceGroupBindSparseInfo {
    fn vk_free(&self) {
        
    }
}