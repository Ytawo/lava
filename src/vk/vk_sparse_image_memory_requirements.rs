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
use vk::vk_sparse_image_format_properties::*;

#[derive(Debug, Clone)]
pub struct VkSparseImageMemoryRequirements {
    pub format_properties: VkSparseImageFormatProperties,
    pub image_mip_tail_first_lod: usize,
    pub image_mip_tail_size: usize,
    pub image_mip_tail_offset: usize,
    pub image_mip_tail_stride: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageMemoryRequirements {
    pub format_properties: RawVkSparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: u64,
    pub image_mip_tail_offset: u64,
    pub image_mip_tail_stride: u64,
}

impl VkWrappedType<RawVkSparseImageMemoryRequirements> for VkSparseImageMemoryRequirements {
    fn vk_to_raw(src: &VkSparseImageMemoryRequirements, dst: &mut RawVkSparseImageMemoryRequirements) {
        dst.format_properties = vk_to_raw_value(&src.format_properties);
        dst.image_mip_tail_first_lod = vk_to_raw_value(&src.image_mip_tail_first_lod);
        dst.image_mip_tail_size = vk_to_raw_value(&src.image_mip_tail_size);
        dst.image_mip_tail_offset = vk_to_raw_value(&src.image_mip_tail_offset);
        dst.image_mip_tail_stride = vk_to_raw_value(&src.image_mip_tail_stride);
    }
}

impl VkRawType<VkSparseImageMemoryRequirements> for RawVkSparseImageMemoryRequirements {
    fn vk_to_wrapped(src: &RawVkSparseImageMemoryRequirements) -> VkSparseImageMemoryRequirements {
        VkSparseImageMemoryRequirements {
            format_properties: RawVkSparseImageFormatProperties::vk_to_wrapped(&src.format_properties),
            image_mip_tail_first_lod: u32::vk_to_wrapped(&src.image_mip_tail_first_lod),
            image_mip_tail_size: u64::vk_to_wrapped(&src.image_mip_tail_size),
            image_mip_tail_offset: u64::vk_to_wrapped(&src.image_mip_tail_offset),
            image_mip_tail_stride: u64::vk_to_wrapped(&src.image_mip_tail_stride),
        }
    }
}

impl Default for VkSparseImageMemoryRequirements {
    fn default() -> VkSparseImageMemoryRequirements {
        VkSparseImageMemoryRequirements {
            format_properties: VkSparseImageFormatProperties::default(),
            image_mip_tail_first_lod: 0,
            image_mip_tail_size: 0,
            image_mip_tail_offset: 0,
            image_mip_tail_stride: 0,
        }
    }
}

impl VkSetup for VkSparseImageMemoryRequirements {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.format_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkSparseImageMemoryRequirements {
    fn vk_free(&mut self) {
        RawVkSparseImageFormatProperties::vk_free(&mut self.format_properties);
    }
}