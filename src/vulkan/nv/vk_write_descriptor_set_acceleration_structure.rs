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
use vulkan::nv::{VkAccelerationStructure,RawVkAccelerationStructure};

/// Wrapper for [VkWriteDescriptorSetAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html).
#[derive(Debug, Clone)]
pub struct VkWriteDescriptorSetAccelerationStructure {
    pub acceleration_structures: Vec<VkAccelerationStructure>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkWriteDescriptorSetAccelerationStructure {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub acceleration_structure_count: u32,
    pub acceleration_structures: *const RawVkAccelerationStructure,
}

impl VkWrappedType<RawVkWriteDescriptorSetAccelerationStructure> for VkWriteDescriptorSetAccelerationStructure {
    fn vk_to_raw(src: &VkWriteDescriptorSetAccelerationStructure, dst: &mut RawVkWriteDescriptorSetAccelerationStructure) {
        dst.s_type = vk_to_raw_value(&VkStructureType::WriteDescriptorSetAccelerationStructureNv);
        dst.next = ptr::null();
        dst.acceleration_structure_count = src.acceleration_structures.len() as u32;
        dst.acceleration_structures = new_ptr_vk_array(&src.acceleration_structures);
    }
}

impl VkRawType<VkWriteDescriptorSetAccelerationStructure> for RawVkWriteDescriptorSetAccelerationStructure {
    fn vk_to_wrapped(src: &RawVkWriteDescriptorSetAccelerationStructure) -> VkWriteDescriptorSetAccelerationStructure {
        VkWriteDescriptorSetAccelerationStructure {
            acceleration_structures: new_vk_array(src.acceleration_structure_count, src.acceleration_structures),
        }
    }
}

impl Default for VkWriteDescriptorSetAccelerationStructure {
    fn default() -> VkWriteDescriptorSetAccelerationStructure {
        VkWriteDescriptorSetAccelerationStructure {
            acceleration_structures: Vec::new(),
        }
    }
}

impl VkSetup for VkWriteDescriptorSetAccelerationStructure {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkWriteDescriptorSetAccelerationStructure {
    fn vk_free(&self) {
        free_ptr(self.acceleration_structures);
    }
}