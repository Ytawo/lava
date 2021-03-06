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
use vulkan::vk::{VkBuffer,RawVkBuffer};

/// Wrapper for [VkIndirectCommandsStreamNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsStreamNV.html).
#[derive(Debug, Clone)]
pub struct VkIndirectCommandsStream {
    pub buffer: VkBuffer,
    pub offset: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkIndirectCommandsStream {
    pub buffer: RawVkBuffer,
    pub offset: u64,
}

impl VkWrappedType<RawVkIndirectCommandsStream> for VkIndirectCommandsStream {
    fn vk_to_raw(src: &VkIndirectCommandsStream, dst: &mut RawVkIndirectCommandsStream) {
        dst.buffer = vk_to_raw_value(&src.buffer);
        dst.offset = vk_to_raw_value(&src.offset);
    }
}

impl VkRawType<VkIndirectCommandsStream> for RawVkIndirectCommandsStream {
    fn vk_to_wrapped(src: &RawVkIndirectCommandsStream) -> VkIndirectCommandsStream {
        VkIndirectCommandsStream {
            buffer: RawVkBuffer::vk_to_wrapped(&src.buffer),
            offset: u64::vk_to_wrapped(&src.offset),
        }
    }
}

impl Default for VkIndirectCommandsStream {
    fn default() -> VkIndirectCommandsStream {
        VkIndirectCommandsStream {
            buffer: Default::default(),
            offset: 0,
        }
    }
}

impl VkSetup for VkIndirectCommandsStream {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.buffer, fn_table);
    }
}

impl VkFree for RawVkIndirectCommandsStream {
    fn vk_free(&self) {
        
    }
}