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
pub struct VkDescriptorPoolSize {
    pub type_: VkDescriptorType,
    pub descriptor_count: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorPoolSize {
    pub type_: RawVkDescriptorType,
    pub descriptor_count: u32,
}

impl VkWrappedType<RawVkDescriptorPoolSize> for VkDescriptorPoolSize {
    fn vk_to_raw(src: &VkDescriptorPoolSize, dst: &mut RawVkDescriptorPoolSize) {
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.descriptor_count = vk_to_raw_value(&src.descriptor_count);
    }
}

impl VkRawType<VkDescriptorPoolSize> for RawVkDescriptorPoolSize {
    fn vk_to_wrapped(src: &RawVkDescriptorPoolSize) -> VkDescriptorPoolSize {
        VkDescriptorPoolSize {
            type_: RawVkDescriptorType::vk_to_wrapped(&src.type_),
            descriptor_count: u32::vk_to_wrapped(&src.descriptor_count),
        }
    }
}

impl Default for VkDescriptorPoolSize {
    fn default() -> VkDescriptorPoolSize {
        VkDescriptorPoolSize {
            type_: VkDescriptorType::default(),
            descriptor_count: 0,
        }
    }
}

impl VkSetup for VkDescriptorPoolSize {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorPoolSize {
    fn vk_free(&mut self) {
        
    }
}