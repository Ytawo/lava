// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_image_aspect_flags::*;

#[repr(C)]
pub struct RawVkImageSubresourceRange {
    aspect_mask: RawVkImageAspectFlags,
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}

#[derive(Debug, Clone)]
pub struct VkImageSubresourceRange {
    pub aspect_mask: VkImageAspectFlags,
    pub base_mip_level: usize,
    pub level_count: usize,
    pub base_array_layer: usize,
    pub layer_count: usize,
}

impl VkRawType<VkImageSubresourceRange> for RawVkImageSubresourceRange {
    fn vk_to_wrapped(src: &RawVkImageSubresourceRange) -> VkImageSubresourceRange {
        VkImageSubresourceRange {
            aspect_mask: RawVkImageAspectFlags::vk_to_wrapped(&src.aspect_mask),
            base_mip_level: u32::vk_to_wrapped(&src.base_mip_level),
            level_count: u32::vk_to_wrapped(&src.level_count),
            base_array_layer: u32::vk_to_wrapped(&src.base_array_layer),
            layer_count: u32::vk_to_wrapped(&src.layer_count),
        }
    }
}

impl VkWrappedType<RawVkImageSubresourceRange> for VkImageSubresourceRange {
    fn vk_to_raw(src: &VkImageSubresourceRange, dst: &mut RawVkImageSubresourceRange) {
        dst.aspect_mask = vk_to_raw_value(&src.aspect_mask);
        dst.base_mip_level = vk_to_raw_value(&src.base_mip_level);
        dst.level_count = vk_to_raw_value(&src.level_count);
        dst.base_array_layer = vk_to_raw_value(&src.base_array_layer);
        dst.layer_count = vk_to_raw_value(&src.layer_count);
    }
}

impl Default for VkImageSubresourceRange {
    fn default() -> VkImageSubresourceRange {
        VkImageSubresourceRange {
            aspect_mask: VkImageAspectFlags::default(),
            base_mip_level: 0,
            level_count: 0,
            base_array_layer: 0,
            layer_count: 0,
        }
    }
}

impl VkSetup for VkImageSubresourceRange {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImageSubresourceRange {
    fn vk_free(&mut self) {
        
    }
}