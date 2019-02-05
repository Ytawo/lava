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

/// Wrapper for [VkBindBufferMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html).
#[derive(Debug, Clone)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
    pub device_indices: Vec<usize>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBindBufferMemoryDeviceGroupInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub device_index_count: u32,
    pub device_indices: *mut u32,
}

impl VkWrappedType<RawVkBindBufferMemoryDeviceGroupInfo> for VkBindBufferMemoryDeviceGroupInfo {
    fn vk_to_raw(src: &VkBindBufferMemoryDeviceGroupInfo, dst: &mut RawVkBindBufferMemoryDeviceGroupInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindBufferMemoryDeviceGroupInfo);
        dst.next = ptr::null_mut();
        dst.device_index_count = src.device_indices.len() as u32;
        dst.device_indices = new_ptr_vk_array(&src.device_indices);
    }
}

impl VkRawType<VkBindBufferMemoryDeviceGroupInfo> for RawVkBindBufferMemoryDeviceGroupInfo {
    fn vk_to_wrapped(src: &RawVkBindBufferMemoryDeviceGroupInfo) -> VkBindBufferMemoryDeviceGroupInfo {
        VkBindBufferMemoryDeviceGroupInfo {
            device_indices: new_vk_array(src.device_index_count, src.device_indices),
        }
    }
}

impl Default for VkBindBufferMemoryDeviceGroupInfo {
    fn default() -> VkBindBufferMemoryDeviceGroupInfo {
        VkBindBufferMemoryDeviceGroupInfo {
            device_indices: Vec::new(),
        }
    }
}

impl VkSetup for VkBindBufferMemoryDeviceGroupInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkBindBufferMemoryDeviceGroupInfo {
    fn vk_free(&self) {
        free_ptr(self.device_indices);
    }
}