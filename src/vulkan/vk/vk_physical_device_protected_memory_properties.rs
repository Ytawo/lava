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

/// Wrapper for [VkPhysicalDeviceProtectedMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
    pub protected_no_fault: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceProtectedMemoryProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub protected_no_fault: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceProtectedMemoryProperties> for VkPhysicalDeviceProtectedMemoryProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceProtectedMemoryProperties, dst: &mut RawVkPhysicalDeviceProtectedMemoryProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceProtectedMemoryProperties);
        dst.next = ptr::null();
        dst.protected_no_fault = vk_to_raw_value(&src.protected_no_fault);
    }
}

impl VkRawType<VkPhysicalDeviceProtectedMemoryProperties> for RawVkPhysicalDeviceProtectedMemoryProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceProtectedMemoryProperties) -> VkPhysicalDeviceProtectedMemoryProperties {
        VkPhysicalDeviceProtectedMemoryProperties {
            protected_no_fault: u32::vk_to_wrapped(&src.protected_no_fault),
        }
    }
}

impl Default for VkPhysicalDeviceProtectedMemoryProperties {
    fn default() -> VkPhysicalDeviceProtectedMemoryProperties {
        VkPhysicalDeviceProtectedMemoryProperties {
            protected_no_fault: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceProtectedMemoryProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceProtectedMemoryProperties {
    fn vk_free(&self) {
        
    }
}