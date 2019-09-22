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

/// Wrapper for [VkPhysicalDeviceShaderSMBuiltinsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeatures {
    pub shader_smbuiltins: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShaderSMBuiltinsFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub shader_smbuiltins: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShaderSMBuiltinsFeatures> for VkPhysicalDeviceShaderSMBuiltinsFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceShaderSMBuiltinsFeatures, dst: &mut RawVkPhysicalDeviceShaderSMBuiltinsFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShaderSmBuiltinsFeaturesNv);
        dst.next = ptr::null_mut();
        dst.shader_smbuiltins = vk_to_raw_value(&src.shader_smbuiltins);
    }
}

impl VkRawType<VkPhysicalDeviceShaderSMBuiltinsFeatures> for RawVkPhysicalDeviceShaderSMBuiltinsFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShaderSMBuiltinsFeatures) -> VkPhysicalDeviceShaderSMBuiltinsFeatures {
        VkPhysicalDeviceShaderSMBuiltinsFeatures {
            shader_smbuiltins: u32::vk_to_wrapped(&src.shader_smbuiltins),
        }
    }
}

impl Default for VkPhysicalDeviceShaderSMBuiltinsFeatures {
    fn default() -> VkPhysicalDeviceShaderSMBuiltinsFeatures {
        VkPhysicalDeviceShaderSMBuiltinsFeatures {
            shader_smbuiltins: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShaderSMBuiltinsFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShaderSMBuiltinsFeatures {
    fn vk_free(&self) {
        
    }
}