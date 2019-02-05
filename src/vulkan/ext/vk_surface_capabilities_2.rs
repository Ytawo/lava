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
use vulkan::khr::{VkSurfaceTransformFlags,RawVkSurfaceTransformFlags};
use vulkan::khr::{VkCompositeAlphaFlags,RawVkCompositeAlphaFlags};
use vulkan::vk::{VkImageUsageFlags,RawVkImageUsageFlags};
use vulkan::ext::{VkSurfaceCounterFlags,RawVkSurfaceCounterFlags};

/// Wrapper for [VkSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSurfaceCapabilities2EXT.html).
#[derive(Debug, Clone)]
pub struct VkSurfaceCapabilities2 {
    pub min_image_count: usize,
    pub max_image_count: usize,
    pub current_extent: VkExtent2D,
    pub min_image_extent: VkExtent2D,
    pub max_image_extent: VkExtent2D,
    pub max_image_array_layers: usize,
    pub supported_transforms: VkSurfaceTransformFlags,
    pub current_transform: VkSurfaceTransformFlags,
    pub supported_composite_alpha: VkCompositeAlphaFlags,
    pub supported_usage_flags: VkImageUsageFlags,
    pub supported_surface_counters: VkSurfaceCounterFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSurfaceCapabilities2 {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: RawVkExtent2D,
    pub min_image_extent: RawVkExtent2D,
    pub max_image_extent: RawVkExtent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: RawVkSurfaceTransformFlags,
    pub current_transform: RawVkSurfaceTransformFlags,
    pub supported_composite_alpha: RawVkCompositeAlphaFlags,
    pub supported_usage_flags: RawVkImageUsageFlags,
    pub supported_surface_counters: RawVkSurfaceCounterFlags,
}

impl VkWrappedType<RawVkSurfaceCapabilities2> for VkSurfaceCapabilities2 {
    fn vk_to_raw(src: &VkSurfaceCapabilities2, dst: &mut RawVkSurfaceCapabilities2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SurfaceCapabilities2Ext);
        dst.next = ptr::null_mut();
        dst.min_image_count = vk_to_raw_value(&src.min_image_count);
        dst.max_image_count = vk_to_raw_value(&src.max_image_count);
        dst.current_extent = vk_to_raw_value(&src.current_extent);
        dst.min_image_extent = vk_to_raw_value(&src.min_image_extent);
        dst.max_image_extent = vk_to_raw_value(&src.max_image_extent);
        dst.max_image_array_layers = vk_to_raw_value(&src.max_image_array_layers);
        dst.supported_transforms = vk_to_raw_value(&src.supported_transforms);
        dst.current_transform = vk_to_raw_value(&src.current_transform);
        dst.supported_composite_alpha = vk_to_raw_value(&src.supported_composite_alpha);
        dst.supported_usage_flags = vk_to_raw_value(&src.supported_usage_flags);
        dst.supported_surface_counters = vk_to_raw_value(&src.supported_surface_counters);
    }
}

impl VkRawType<VkSurfaceCapabilities2> for RawVkSurfaceCapabilities2 {
    fn vk_to_wrapped(src: &RawVkSurfaceCapabilities2) -> VkSurfaceCapabilities2 {
        VkSurfaceCapabilities2 {
            min_image_count: u32::vk_to_wrapped(&src.min_image_count),
            max_image_count: u32::vk_to_wrapped(&src.max_image_count),
            current_extent: RawVkExtent2D::vk_to_wrapped(&src.current_extent),
            min_image_extent: RawVkExtent2D::vk_to_wrapped(&src.min_image_extent),
            max_image_extent: RawVkExtent2D::vk_to_wrapped(&src.max_image_extent),
            max_image_array_layers: u32::vk_to_wrapped(&src.max_image_array_layers),
            supported_transforms: RawVkSurfaceTransformFlags::vk_to_wrapped(&src.supported_transforms),
            current_transform: RawVkSurfaceTransformFlags::vk_to_wrapped(&src.current_transform),
            supported_composite_alpha: RawVkCompositeAlphaFlags::vk_to_wrapped(&src.supported_composite_alpha),
            supported_usage_flags: RawVkImageUsageFlags::vk_to_wrapped(&src.supported_usage_flags),
            supported_surface_counters: RawVkSurfaceCounterFlags::vk_to_wrapped(&src.supported_surface_counters),
        }
    }
}

impl Default for VkSurfaceCapabilities2 {
    fn default() -> VkSurfaceCapabilities2 {
        VkSurfaceCapabilities2 {
            min_image_count: 0,
            max_image_count: 0,
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: 0,
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
            supported_surface_counters: Default::default(),
        }
    }
}

impl VkSetup for VkSurfaceCapabilities2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.current_extent, fn_table);
        VkSetup::vk_setup(&mut self.min_image_extent, fn_table);
        VkSetup::vk_setup(&mut self.max_image_extent, fn_table);
    }
}

impl VkFree for RawVkSurfaceCapabilities2 {
    fn vk_free(&self) {
        
    }
}