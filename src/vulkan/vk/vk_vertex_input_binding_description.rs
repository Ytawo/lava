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
use vulkan::vk::{VkVertexInputRate,RawVkVertexInputRate};

/// Wrapper for [VkVertexInputBindingDescription](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkVertexInputBindingDescription.html).
#[derive(Debug, Clone)]
pub struct VkVertexInputBindingDescription {
    pub binding: usize,
    pub stride: usize,
    pub input_rate: VkVertexInputRate,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: RawVkVertexInputRate,
}

impl VkWrappedType<RawVkVertexInputBindingDescription> for VkVertexInputBindingDescription {
    fn vk_to_raw(src: &VkVertexInputBindingDescription, dst: &mut RawVkVertexInputBindingDescription) {
        dst.binding = vk_to_raw_value(&src.binding);
        dst.stride = vk_to_raw_value(&src.stride);
        dst.input_rate = vk_to_raw_value(&src.input_rate);
    }
}

impl VkRawType<VkVertexInputBindingDescription> for RawVkVertexInputBindingDescription {
    fn vk_to_wrapped(src: &RawVkVertexInputBindingDescription) -> VkVertexInputBindingDescription {
        VkVertexInputBindingDescription {
            binding: u32::vk_to_wrapped(&src.binding),
            stride: u32::vk_to_wrapped(&src.stride),
            input_rate: RawVkVertexInputRate::vk_to_wrapped(&src.input_rate),
        }
    }
}

impl Default for VkVertexInputBindingDescription {
    fn default() -> VkVertexInputBindingDescription {
        VkVertexInputBindingDescription {
            binding: 0,
            stride: 0,
            input_rate: Default::default(),
        }
    }
}

impl VkSetup for VkVertexInputBindingDescription {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkVertexInputBindingDescription {
    fn vk_free(&self) {
        
    }
}