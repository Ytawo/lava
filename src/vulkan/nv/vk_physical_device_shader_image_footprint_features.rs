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

/// Wrapper for [VkPhysicalDeviceShaderImageFootprintFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShaderImageFootprintFeatures {
    pub image_footprint: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShaderImageFootprintFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image_footprint: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShaderImageFootprintFeatures> for VkPhysicalDeviceShaderImageFootprintFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceShaderImageFootprintFeatures, dst: &mut RawVkPhysicalDeviceShaderImageFootprintFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShaderImageFootprintFeaturesNv);
        dst.next = ptr::null();
        dst.image_footprint = vk_to_raw_value(&src.image_footprint);
    }
}

impl VkRawType<VkPhysicalDeviceShaderImageFootprintFeatures> for RawVkPhysicalDeviceShaderImageFootprintFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShaderImageFootprintFeatures) -> VkPhysicalDeviceShaderImageFootprintFeatures {
        VkPhysicalDeviceShaderImageFootprintFeatures {
            image_footprint: u32::vk_to_wrapped(&src.image_footprint),
        }
    }
}

impl Default for VkPhysicalDeviceShaderImageFootprintFeatures {
    fn default() -> VkPhysicalDeviceShaderImageFootprintFeatures {
        VkPhysicalDeviceShaderImageFootprintFeatures {
            image_footprint: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShaderImageFootprintFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShaderImageFootprintFeatures {
    fn vk_free(&self) {
        
    }
}