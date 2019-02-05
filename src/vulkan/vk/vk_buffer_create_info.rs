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
use vulkan::vk::{VkBufferCreateFlags,RawVkBufferCreateFlags};
use vulkan::vk::{VkBufferUsageFlags,RawVkBufferUsageFlags};
use vulkan::vk::{VkSharingMode,RawVkSharingMode};

/// Wrapper for [VkBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkBufferCreateInfo {
    pub flags: VkBufferCreateFlags,
    pub size: usize,
    pub usage: VkBufferUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_family_indices: Vec<usize>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkBufferCreateFlags,
    pub size: u64,
    pub usage: RawVkBufferUsageFlags,
    pub sharing_mode: RawVkSharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
}

impl VkWrappedType<RawVkBufferCreateInfo> for VkBufferCreateInfo {
    fn vk_to_raw(src: &VkBufferCreateInfo, dst: &mut RawVkBufferCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.size = vk_to_raw_value(&src.size);
        dst.usage = vk_to_raw_value(&src.usage);
        dst.sharing_mode = vk_to_raw_value(&src.sharing_mode);
        dst.queue_family_index_count = src.queue_family_indices.len() as u32;
        dst.queue_family_indices = new_ptr_vk_array(&src.queue_family_indices);
    }
}

impl VkRawType<VkBufferCreateInfo> for RawVkBufferCreateInfo {
    fn vk_to_wrapped(src: &RawVkBufferCreateInfo) -> VkBufferCreateInfo {
        VkBufferCreateInfo {
            flags: RawVkBufferCreateFlags::vk_to_wrapped(&src.flags),
            size: u64::vk_to_wrapped(&src.size),
            usage: RawVkBufferUsageFlags::vk_to_wrapped(&src.usage),
            sharing_mode: RawVkSharingMode::vk_to_wrapped(&src.sharing_mode),
            queue_family_indices: new_vk_array(src.queue_family_index_count, src.queue_family_indices),
        }
    }
}

impl Default for VkBufferCreateInfo {
    fn default() -> VkBufferCreateInfo {
        VkBufferCreateInfo {
            flags: Default::default(),
            size: 0,
            usage: Default::default(),
            sharing_mode: Default::default(),
            queue_family_indices: Vec::new(),
        }
    }
}

impl VkSetup for VkBufferCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkBufferCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.queue_family_indices);
    }
}