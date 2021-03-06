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
use vulkan::ext::{VkConditionalRenderingFlags,RawVkConditionalRenderingFlags};

/// Wrapper for [VkConditionalRenderingBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkConditionalRenderingBeginInfo {
    pub buffer: VkBuffer,
    pub offset: usize,
    pub flags: VkConditionalRenderingFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkConditionalRenderingBeginInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub buffer: RawVkBuffer,
    pub offset: u64,
    pub flags: RawVkConditionalRenderingFlags,
}

impl VkWrappedType<RawVkConditionalRenderingBeginInfo> for VkConditionalRenderingBeginInfo {
    fn vk_to_raw(src: &VkConditionalRenderingBeginInfo, dst: &mut RawVkConditionalRenderingBeginInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ConditionalRenderingBeginInfoExt);
        dst.next = ptr::null_mut();
        dst.buffer = vk_to_raw_value(&src.buffer);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl VkRawType<VkConditionalRenderingBeginInfo> for RawVkConditionalRenderingBeginInfo {
    fn vk_to_wrapped(src: &RawVkConditionalRenderingBeginInfo) -> VkConditionalRenderingBeginInfo {
        VkConditionalRenderingBeginInfo {
            buffer: RawVkBuffer::vk_to_wrapped(&src.buffer),
            offset: u64::vk_to_wrapped(&src.offset),
            flags: RawVkConditionalRenderingFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl Default for VkConditionalRenderingBeginInfo {
    fn default() -> VkConditionalRenderingBeginInfo {
        VkConditionalRenderingBeginInfo {
            buffer: Default::default(),
            offset: 0,
            flags: Default::default(),
        }
    }
}

impl VkSetup for VkConditionalRenderingBeginInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.buffer, fn_table);
    }
}

impl VkFree for RawVkConditionalRenderingBeginInfo {
    fn vk_free(&self) {
        
    }
}