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
use vulkan::vk::{VkDeviceMemory,RawVkDeviceMemory};
use vulkan::vk::{VkSparseMemoryBindFlags,RawVkSparseMemoryBindFlags};

/// Wrapper for [VkSparseMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSparseMemoryBind.html).
#[derive(Debug, Clone)]
pub struct VkSparseMemoryBind<'a> {
    pub resource_offset: usize,
    pub size: usize,
    pub memory: Option<&'a VkDeviceMemory>,
    pub memory_offset: usize,
    pub flags: VkSparseMemoryBindFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseMemoryBind {
    pub resource_offset: u64,
    pub size: u64,
    pub memory: RawVkDeviceMemory,
    pub memory_offset: u64,
    pub flags: RawVkSparseMemoryBindFlags,
}

impl<'a> VkWrappedType<RawVkSparseMemoryBind> for VkSparseMemoryBind<'a> {
    fn vk_to_raw(src: &VkSparseMemoryBind, dst: &mut RawVkSparseMemoryBind) {
        dst.resource_offset = vk_to_raw_value(&src.resource_offset);
        dst.size = vk_to_raw_value(&src.size);
        dst.memory = if src.memory.is_some() { vk_to_raw_value(src.memory.unwrap()) } else { 0 };
        dst.memory_offset = vk_to_raw_value(&src.memory_offset);
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl Default for VkSparseMemoryBind<'static> {
    fn default() -> VkSparseMemoryBind<'static> {
        VkSparseMemoryBind {
            resource_offset: 0,
            size: 0,
            memory: None,
            memory_offset: 0,
            flags: VkSparseMemoryBindFlags::default(),
        }
    }
}

impl<'a> VkSetup for VkSparseMemoryBind<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSparseMemoryBind {
    fn vk_free(&mut self) {
        
    }
}