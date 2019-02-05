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

/// Wrapper for [VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeatures {
    pub advanced_blend_coherent_operations: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceBlendOperationAdvancedFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub advanced_blend_coherent_operations: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceBlendOperationAdvancedFeatures> for VkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceBlendOperationAdvancedFeatures, dst: &mut RawVkPhysicalDeviceBlendOperationAdvancedFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceBlendOperationAdvancedFeaturesExt);
        dst.next = ptr::null();
        dst.advanced_blend_coherent_operations = vk_to_raw_value(&src.advanced_blend_coherent_operations);
    }
}

impl VkRawType<VkPhysicalDeviceBlendOperationAdvancedFeatures> for RawVkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceBlendOperationAdvancedFeatures) -> VkPhysicalDeviceBlendOperationAdvancedFeatures {
        VkPhysicalDeviceBlendOperationAdvancedFeatures {
            advanced_blend_coherent_operations: u32::vk_to_wrapped(&src.advanced_blend_coherent_operations),
        }
    }
}

impl Default for VkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn default() -> VkPhysicalDeviceBlendOperationAdvancedFeatures {
        VkPhysicalDeviceBlendOperationAdvancedFeatures {
            advanced_blend_coherent_operations: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_free(&self) {
        
    }
}