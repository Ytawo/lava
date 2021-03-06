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

/// Wrapper for [VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMemoryPriorityFeatures {
    pub memory_priority: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMemoryPriorityFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub memory_priority: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceMemoryPriorityFeatures> for VkPhysicalDeviceMemoryPriorityFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceMemoryPriorityFeatures, dst: &mut RawVkPhysicalDeviceMemoryPriorityFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMemoryPriorityFeaturesExt);
        dst.next = ptr::null_mut();
        dst.memory_priority = vk_to_raw_value(&src.memory_priority);
    }
}

impl VkRawType<VkPhysicalDeviceMemoryPriorityFeatures> for RawVkPhysicalDeviceMemoryPriorityFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMemoryPriorityFeatures) -> VkPhysicalDeviceMemoryPriorityFeatures {
        VkPhysicalDeviceMemoryPriorityFeatures {
            memory_priority: u32::vk_to_wrapped(&src.memory_priority),
        }
    }
}

impl Default for VkPhysicalDeviceMemoryPriorityFeatures {
    fn default() -> VkPhysicalDeviceMemoryPriorityFeatures {
        VkPhysicalDeviceMemoryPriorityFeatures {
            memory_priority: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceMemoryPriorityFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMemoryPriorityFeatures {
    fn vk_free(&self) {
        
    }
}