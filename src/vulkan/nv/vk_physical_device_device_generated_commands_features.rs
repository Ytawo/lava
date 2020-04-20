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

/// Wrapper for [VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    pub device_generated_commands: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub device_generated_commands: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDeviceGeneratedCommandsFeatures> for VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceDeviceGeneratedCommandsFeatures, dst: &mut RawVkPhysicalDeviceDeviceGeneratedCommandsFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDeviceGeneratedCommandsFeaturesNv);
        dst.next = ptr::null_mut();
        dst.device_generated_commands = vk_to_raw_value(&src.device_generated_commands);
    }
}

impl VkRawType<VkPhysicalDeviceDeviceGeneratedCommandsFeatures> for RawVkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDeviceGeneratedCommandsFeatures) -> VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
        VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
            device_generated_commands: u32::vk_to_wrapped(&src.device_generated_commands),
        }
    }
}

impl Default for VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    fn default() -> VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
        VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
            device_generated_commands: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDeviceGeneratedCommandsFeatures {
    fn vk_free(&self) {
        
    }
}