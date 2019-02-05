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

/// Wrapper for [VkPhysicalDeviceProtectedMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
    pub protected_memory: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceProtectedMemoryFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub protected_memory: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceProtectedMemoryFeatures> for VkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceProtectedMemoryFeatures, dst: &mut RawVkPhysicalDeviceProtectedMemoryFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceProtectedMemoryFeatures);
        dst.next = ptr::null();
        dst.protected_memory = vk_to_raw_value(&src.protected_memory);
    }
}

impl VkRawType<VkPhysicalDeviceProtectedMemoryFeatures> for RawVkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceProtectedMemoryFeatures) -> VkPhysicalDeviceProtectedMemoryFeatures {
        VkPhysicalDeviceProtectedMemoryFeatures {
            protected_memory: u32::vk_to_wrapped(&src.protected_memory),
        }
    }
}

impl Default for VkPhysicalDeviceProtectedMemoryFeatures {
    fn default() -> VkPhysicalDeviceProtectedMemoryFeatures {
        VkPhysicalDeviceProtectedMemoryFeatures {
            protected_memory: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_free(&self) {
        
    }
}