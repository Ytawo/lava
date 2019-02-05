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
use vulkan::vk::{VkImage,RawVkImage};
use vulkan::vk::{VkDeviceMemory,RawVkDeviceMemory};

/// Wrapper for [VkBindImageMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBindImageMemoryInfo.html).
#[derive(Debug, Clone)]
pub struct VkBindImageMemoryInfo<'a, 'b> {
    pub image: &'a VkImage,
    pub memory: &'b VkDeviceMemory,
    pub memory_offset: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBindImageMemoryInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image: RawVkImage,
    pub memory: RawVkDeviceMemory,
    pub memory_offset: u64,
}

impl<'a, 'b> VkWrappedType<RawVkBindImageMemoryInfo> for VkBindImageMemoryInfo<'a, 'b> {
    fn vk_to_raw(src: &VkBindImageMemoryInfo, dst: &mut RawVkBindImageMemoryInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindImageMemoryInfo);
        dst.next = ptr::null();
        dst.image = vk_to_raw_value(src.image);
        dst.memory = vk_to_raw_value(src.memory);
        dst.memory_offset = vk_to_raw_value(&src.memory_offset);
    }
}

impl Default for VkBindImageMemoryInfo<'static, 'static> {
    fn default() -> VkBindImageMemoryInfo<'static, 'static> {
        VkBindImageMemoryInfo {
            image: vk_null_ref(),
            memory: vk_null_ref(),
            memory_offset: 0,
        }
    }
}

impl<'a, 'b> VkSetup for VkBindImageMemoryInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBindImageMemoryInfo {
    fn vk_free(&mut self) {
        
    }
}