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
use vulkan::vk::{VkPhysicalDeviceMemoryProperties,RawVkPhysicalDeviceMemoryProperties};

/// Wrapper for [VkPhysicalDeviceMemoryProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMemoryProperties2 {
    pub memory_properties: VkPhysicalDeviceMemoryProperties,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMemoryProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub memory_properties: RawVkPhysicalDeviceMemoryProperties,
}

impl VkWrappedType<RawVkPhysicalDeviceMemoryProperties2> for VkPhysicalDeviceMemoryProperties2 {
    fn vk_to_raw(src: &VkPhysicalDeviceMemoryProperties2, dst: &mut RawVkPhysicalDeviceMemoryProperties2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMemoryProperties2);
        dst.next = ptr::null_mut();
        dst.memory_properties = vk_to_raw_value(&src.memory_properties);
    }
}

impl VkRawType<VkPhysicalDeviceMemoryProperties2> for RawVkPhysicalDeviceMemoryProperties2 {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMemoryProperties2) -> VkPhysicalDeviceMemoryProperties2 {
        VkPhysicalDeviceMemoryProperties2 {
            memory_properties: RawVkPhysicalDeviceMemoryProperties::vk_to_wrapped(&src.memory_properties),
        }
    }
}

impl Default for VkPhysicalDeviceMemoryProperties2 {
    fn default() -> VkPhysicalDeviceMemoryProperties2 {
        VkPhysicalDeviceMemoryProperties2 {
            memory_properties: Default::default(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceMemoryProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.memory_properties, fn_table);
    }
}

impl VkFree for RawVkPhysicalDeviceMemoryProperties2 {
    fn vk_free(&self) {
        
    }
}