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

/// Wrapper for [VkPhysicalDeviceShadingRateImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShadingRateImageFeatures {
    pub shading_rate_image: bool,
    pub shading_rate_coarse_sample_order: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShadingRateImageFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub shading_rate_image: u32,
    pub shading_rate_coarse_sample_order: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShadingRateImageFeatures> for VkPhysicalDeviceShadingRateImageFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceShadingRateImageFeatures, dst: &mut RawVkPhysicalDeviceShadingRateImageFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShadingRateImageFeaturesNv);
        dst.next = ptr::null();
        dst.shading_rate_image = vk_to_raw_value(&src.shading_rate_image);
        dst.shading_rate_coarse_sample_order = vk_to_raw_value(&src.shading_rate_coarse_sample_order);
    }
}

impl VkRawType<VkPhysicalDeviceShadingRateImageFeatures> for RawVkPhysicalDeviceShadingRateImageFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShadingRateImageFeatures) -> VkPhysicalDeviceShadingRateImageFeatures {
        VkPhysicalDeviceShadingRateImageFeatures {
            shading_rate_image: u32::vk_to_wrapped(&src.shading_rate_image),
            shading_rate_coarse_sample_order: u32::vk_to_wrapped(&src.shading_rate_coarse_sample_order),
        }
    }
}

impl Default for VkPhysicalDeviceShadingRateImageFeatures {
    fn default() -> VkPhysicalDeviceShadingRateImageFeatures {
        VkPhysicalDeviceShadingRateImageFeatures {
            shading_rate_image: false,
            shading_rate_coarse_sample_order: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShadingRateImageFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShadingRateImageFeatures {
    fn vk_free(&self) {
        
    }
}