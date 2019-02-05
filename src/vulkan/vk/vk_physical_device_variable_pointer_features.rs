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

/// Wrapper for [VkPhysicalDeviceVariablePointerFeatures](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceVariablePointerFeatures.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceVariablePointerFeatures {
    pub variable_pointers_storage_buffer: bool,
    pub variable_pointers: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceVariablePointerFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub variable_pointers_storage_buffer: u32,
    pub variable_pointers: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceVariablePointerFeatures> for VkPhysicalDeviceVariablePointerFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceVariablePointerFeatures, dst: &mut RawVkPhysicalDeviceVariablePointerFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceVariablePointerFeatures);
        dst.next = ptr::null();
        dst.variable_pointers_storage_buffer = vk_to_raw_value(&src.variable_pointers_storage_buffer);
        dst.variable_pointers = vk_to_raw_value(&src.variable_pointers);
    }
}

impl VkRawType<VkPhysicalDeviceVariablePointerFeatures> for RawVkPhysicalDeviceVariablePointerFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceVariablePointerFeatures) -> VkPhysicalDeviceVariablePointerFeatures {
        VkPhysicalDeviceVariablePointerFeatures {
            variable_pointers_storage_buffer: u32::vk_to_wrapped(&src.variable_pointers_storage_buffer),
            variable_pointers: u32::vk_to_wrapped(&src.variable_pointers),
        }
    }
}

impl Default for VkPhysicalDeviceVariablePointerFeatures {
    fn default() -> VkPhysicalDeviceVariablePointerFeatures {
        VkPhysicalDeviceVariablePointerFeatures {
            variable_pointers_storage_buffer: false,
            variable_pointers: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceVariablePointerFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceVariablePointerFeatures {
    fn vk_free(&self) {
        
    }
}