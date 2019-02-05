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
use vulkan::vk::{VkDescriptorSet,RawVkDescriptorSet};
use vulkan::vk::{VkDescriptorType,RawVkDescriptorType};
use vulkan::vk::{VkDescriptorImageInfo,RawVkDescriptorImageInfo};
use vulkan::vk::{VkDescriptorBufferInfo,RawVkDescriptorBufferInfo};
use vulkan::vk::{VkBufferView,RawVkBufferView};

/// Wrapper for [VkWriteDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkWriteDescriptorSet.html).
#[derive(Debug, Clone)]
pub struct VkWriteDescriptorSet {
    pub dst_set: VkDescriptorSet,
    pub dst_binding: usize,
    pub dst_array_element: usize,
    pub descriptor_type: VkDescriptorType,
    pub image_info: Vec<VkDescriptorImageInfo>,
    pub buffer_info: Vec<VkDescriptorBufferInfo>,
    pub texel_buffer_view: Vec<VkBufferView>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkWriteDescriptorSet {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub dst_set: RawVkDescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: RawVkDescriptorType,
    pub image_info: *const RawVkDescriptorImageInfo,
    pub buffer_info: *const RawVkDescriptorBufferInfo,
    pub texel_buffer_view: *const RawVkBufferView,
}

impl VkWrappedType<RawVkWriteDescriptorSet> for VkWriteDescriptorSet {
    fn vk_to_raw(src: &VkWriteDescriptorSet, dst: &mut RawVkWriteDescriptorSet) {
        dst.s_type = vk_to_raw_value(&VkStructureType::WriteDescriptorSet);
        dst.next = ptr::null();
        dst.dst_set = vk_to_raw_value(&src.dst_set);
        dst.dst_binding = vk_to_raw_value(&src.dst_binding);
        dst.dst_array_element = vk_to_raw_value(&src.dst_array_element);
        dst.descriptor_count = cmp::max(cmp::max(src.image_info.len(), src.buffer_info.len()), src.texel_buffer_view.len()) as u32;
        dst.descriptor_type = vk_to_raw_value(&src.descriptor_type);
        dst.image_info = new_ptr_vk_array(&src.image_info);
        dst.buffer_info = new_ptr_vk_array(&src.buffer_info);
        dst.texel_buffer_view = new_ptr_vk_array(&src.texel_buffer_view);
    }
}

impl VkRawType<VkWriteDescriptorSet> for RawVkWriteDescriptorSet {
    fn vk_to_wrapped(src: &RawVkWriteDescriptorSet) -> VkWriteDescriptorSet {
        VkWriteDescriptorSet {
            dst_set: RawVkDescriptorSet::vk_to_wrapped(&src.dst_set),
            dst_binding: u32::vk_to_wrapped(&src.dst_binding),
            dst_array_element: u32::vk_to_wrapped(&src.dst_array_element),
            descriptor_type: RawVkDescriptorType::vk_to_wrapped(&src.descriptor_type),
            image_info: new_vk_array(src.descriptor_count, src.image_info),
            buffer_info: new_vk_array(src.descriptor_count, src.buffer_info),
            texel_buffer_view: new_vk_array(src.descriptor_count, src.texel_buffer_view),
        }
    }
}

impl Default for VkWriteDescriptorSet {
    fn default() -> VkWriteDescriptorSet {
        VkWriteDescriptorSet {
            dst_set: Default::default(),
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_type: Default::default(),
            image_info: Vec::new(),
            buffer_info: Vec::new(),
            texel_buffer_view: Vec::new(),
        }
    }
}

impl VkSetup for VkWriteDescriptorSet {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.dst_set, fn_table);
    }
}

impl VkFree for RawVkWriteDescriptorSet {
    fn vk_free(&self) {
        free_vk_ptr_array(self.descriptor_count as usize, self.image_info);
        free_vk_ptr_array(self.descriptor_count as usize, self.buffer_info);
        free_ptr(self.texel_buffer_view);
    }
}