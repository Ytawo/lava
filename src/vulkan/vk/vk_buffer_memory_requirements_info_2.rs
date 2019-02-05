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
use vulkan::vk::{VkBuffer,RawVkBuffer};

/// Wrapper for [VkBufferMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferMemoryRequirementsInfo2.html).
#[derive(Debug, Clone)]
pub struct VkBufferMemoryRequirementsInfo2 {
    pub buffer: VkBuffer,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferMemoryRequirementsInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub buffer: RawVkBuffer,
}

impl VkWrappedType<RawVkBufferMemoryRequirementsInfo2> for VkBufferMemoryRequirementsInfo2 {
    fn vk_to_raw(src: &VkBufferMemoryRequirementsInfo2, dst: &mut RawVkBufferMemoryRequirementsInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferMemoryRequirementsInfo2);
        dst.next = ptr::null();
        dst.buffer = vk_to_raw_value(&src.buffer);
    }
}

impl VkRawType<VkBufferMemoryRequirementsInfo2> for RawVkBufferMemoryRequirementsInfo2 {
    fn vk_to_wrapped(src: &RawVkBufferMemoryRequirementsInfo2) -> VkBufferMemoryRequirementsInfo2 {
        VkBufferMemoryRequirementsInfo2 {
            buffer: RawVkBuffer::vk_to_wrapped(&src.buffer),
        }
    }
}

impl Default for VkBufferMemoryRequirementsInfo2 {
    fn default() -> VkBufferMemoryRequirementsInfo2 {
        VkBufferMemoryRequirementsInfo2 {
            buffer: Default::default(),
        }
    }
}

impl VkSetup for VkBufferMemoryRequirementsInfo2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.buffer, fn_table);
    }
}

impl VkFree for RawVkBufferMemoryRequirementsInfo2 {
    fn vk_free(&self) {
        
    }
}