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
use vulkan::vk::{VkExtent2D,RawVkExtent2D};

/// Wrapper for [VkPhysicalDeviceShadingRateImagePropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShadingRateImageProperties {
    pub shading_rate_texel_size: VkExtent2D,
    pub shading_rate_palette_size: usize,
    pub shading_rate_max_coarse_samples: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShadingRateImageProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub shading_rate_texel_size: RawVkExtent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShadingRateImageProperties> for VkPhysicalDeviceShadingRateImageProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceShadingRateImageProperties, dst: &mut RawVkPhysicalDeviceShadingRateImageProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShadingRateImagePropertiesNv);
        dst.next = ptr::null_mut();
        dst.shading_rate_texel_size = vk_to_raw_value(&src.shading_rate_texel_size);
        dst.shading_rate_palette_size = vk_to_raw_value(&src.shading_rate_palette_size);
        dst.shading_rate_max_coarse_samples = vk_to_raw_value(&src.shading_rate_max_coarse_samples);
    }
}

impl VkRawType<VkPhysicalDeviceShadingRateImageProperties> for RawVkPhysicalDeviceShadingRateImageProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShadingRateImageProperties) -> VkPhysicalDeviceShadingRateImageProperties {
        VkPhysicalDeviceShadingRateImageProperties {
            shading_rate_texel_size: RawVkExtent2D::vk_to_wrapped(&src.shading_rate_texel_size),
            shading_rate_palette_size: u32::vk_to_wrapped(&src.shading_rate_palette_size),
            shading_rate_max_coarse_samples: u32::vk_to_wrapped(&src.shading_rate_max_coarse_samples),
        }
    }
}

impl Default for VkPhysicalDeviceShadingRateImageProperties {
    fn default() -> VkPhysicalDeviceShadingRateImageProperties {
        VkPhysicalDeviceShadingRateImageProperties {
            shading_rate_texel_size: Default::default(),
            shading_rate_palette_size: 0,
            shading_rate_max_coarse_samples: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShadingRateImageProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.shading_rate_texel_size, fn_table);
    }
}

impl VkFree for RawVkPhysicalDeviceShadingRateImageProperties {
    fn vk_free(&self) {
        
    }
}