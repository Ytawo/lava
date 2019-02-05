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
use vulkan::vk::{VkSpecializationMapEntry,RawVkSpecializationMapEntry};

/// Wrapper for [VkSpecializationInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSpecializationInfo.html).
#[derive(Debug, Clone)]
pub struct VkSpecializationInfo<'a> {
    pub map_entries: Vec<VkSpecializationMapEntry>,
    pub data: &'a [c_void],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSpecializationInfo {
    pub map_entry_count: u32,
    pub map_entries: *const RawVkSpecializationMapEntry,
    pub data_size: usize,
    pub data: *const c_void,
}

impl<'a> VkWrappedType<RawVkSpecializationInfo> for VkSpecializationInfo<'a> {
    fn vk_to_raw(src: &VkSpecializationInfo, dst: &mut RawVkSpecializationInfo) {
        dst.map_entry_count = src.map_entries.len() as u32;
        dst.map_entries = new_ptr_vk_array(&src.map_entries);
        dst.data_size = src.data.len();
        dst.data = src.data.as_ptr();
    }
}

impl Default for VkSpecializationInfo<'static> {
    fn default() -> VkSpecializationInfo<'static> {
        VkSpecializationInfo {
            map_entries: Vec::new(),
            data: &[],
        }
    }
}

impl<'a> VkSetup for VkSpecializationInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSpecializationInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.map_entry_count as usize, self.map_entries);
    }
}