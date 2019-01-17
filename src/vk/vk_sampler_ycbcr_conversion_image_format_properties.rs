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
use vk::vk_structure_type::*;

#[derive(Debug, Clone)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
    pub combined_image_sampler_descriptor_count: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSamplerYcbcrConversionImageFormatProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub combined_image_sampler_descriptor_count: u32,
}

impl VkWrappedType<RawVkSamplerYcbcrConversionImageFormatProperties> for VkSamplerYcbcrConversionImageFormatProperties {
    fn vk_to_raw(src: &VkSamplerYcbcrConversionImageFormatProperties, dst: &mut RawVkSamplerYcbcrConversionImageFormatProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SamplerYcbcrConversionImageFormatProperties);
        dst.next = ptr::null();
        dst.combined_image_sampler_descriptor_count = vk_to_raw_value(&src.combined_image_sampler_descriptor_count);
    }
}

impl VkRawType<VkSamplerYcbcrConversionImageFormatProperties> for RawVkSamplerYcbcrConversionImageFormatProperties {
    fn vk_to_wrapped(src: &RawVkSamplerYcbcrConversionImageFormatProperties) -> VkSamplerYcbcrConversionImageFormatProperties {
        VkSamplerYcbcrConversionImageFormatProperties {
            combined_image_sampler_descriptor_count: u32::vk_to_wrapped(&src.combined_image_sampler_descriptor_count),
        }
    }
}

impl Default for VkSamplerYcbcrConversionImageFormatProperties {
    fn default() -> VkSamplerYcbcrConversionImageFormatProperties {
        VkSamplerYcbcrConversionImageFormatProperties {
            combined_image_sampler_descriptor_count: 0,
        }
    }
}

impl VkSetup for VkSamplerYcbcrConversionImageFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSamplerYcbcrConversionImageFormatProperties {
    fn vk_free(&mut self) {
        
    }
}