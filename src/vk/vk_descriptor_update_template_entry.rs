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
use vk::vk_descriptor_type::*;

#[derive(Debug, Clone)]
pub struct VkDescriptorUpdateTemplateEntry {
    pub dst_binding: usize,
    pub dst_array_element: usize,
    pub descriptor_count: usize,
    pub descriptor_type: VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: RawVkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

impl VkWrappedType<RawVkDescriptorUpdateTemplateEntry> for VkDescriptorUpdateTemplateEntry {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplateEntry, dst: &mut RawVkDescriptorUpdateTemplateEntry) {
        dst.dst_binding = vk_to_raw_value(&src.dst_binding);
        dst.dst_array_element = vk_to_raw_value(&src.dst_array_element);
        dst.descriptor_count = vk_to_raw_value(&src.descriptor_count);
        dst.descriptor_type = vk_to_raw_value(&src.descriptor_type);
        dst.offset = src.offset;
        dst.stride = src.stride;
    }
}

impl VkRawType<VkDescriptorUpdateTemplateEntry> for RawVkDescriptorUpdateTemplateEntry {
    fn vk_to_wrapped(src: &RawVkDescriptorUpdateTemplateEntry) -> VkDescriptorUpdateTemplateEntry {
        VkDescriptorUpdateTemplateEntry {
            dst_binding: u32::vk_to_wrapped(&src.dst_binding),
            dst_array_element: u32::vk_to_wrapped(&src.dst_array_element),
            descriptor_count: u32::vk_to_wrapped(&src.descriptor_count),
            descriptor_type: RawVkDescriptorType::vk_to_wrapped(&src.descriptor_type),
            offset: src.offset,
            stride: src.stride,
        }
    }
}

impl Default for VkDescriptorUpdateTemplateEntry {
    fn default() -> VkDescriptorUpdateTemplateEntry {
        VkDescriptorUpdateTemplateEntry {
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 0,
            descriptor_type: VkDescriptorType::default(),
            offset: 0,
            stride: 0,
        }
    }
}

impl VkSetup for VkDescriptorUpdateTemplateEntry {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorUpdateTemplateEntry {
    fn vk_free(&mut self) {
        
    }
}