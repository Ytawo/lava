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

/// Wrapper for [VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub shader_subgroup_extended_types: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub shader_subgroup_extended_types: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShaderSubgroupExtendedTypesFeatures> for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures, dst: &mut RawVkPhysicalDeviceShaderSubgroupExtendedTypesFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKhr);
        dst.next = ptr::null_mut();
        dst.shader_subgroup_extended_types = vk_to_raw_value(&src.shader_subgroup_extended_types);
    }
}

impl VkRawType<VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures> for RawVkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShaderSubgroupExtendedTypesFeatures) -> VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
        VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
            shader_subgroup_extended_types: u32::vk_to_wrapped(&src.shader_subgroup_extended_types),
        }
    }
}

impl Default for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn default() -> VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
        VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
            shader_subgroup_extended_types: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn vk_free(&self) {
        
    }
}