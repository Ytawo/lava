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

/// Wrapper for [VkExtent3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html).
#[derive(Debug, Clone)]
pub struct VkExtent3D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl VkWrappedType<RawVkExtent3D> for VkExtent3D {
    fn vk_to_raw(src: &VkExtent3D, dst: &mut RawVkExtent3D) {
        dst.width = vk_to_raw_value(&src.width);
        dst.height = vk_to_raw_value(&src.height);
        dst.depth = vk_to_raw_value(&src.depth);
    }
}

impl VkRawType<VkExtent3D> for RawVkExtent3D {
    fn vk_to_wrapped(src: &RawVkExtent3D) -> VkExtent3D {
        VkExtent3D {
            width: u32::vk_to_wrapped(&src.width),
            height: u32::vk_to_wrapped(&src.height),
            depth: u32::vk_to_wrapped(&src.depth),
        }
    }
}

impl Default for VkExtent3D {
    fn default() -> VkExtent3D {
        VkExtent3D {
            width: 0,
            height: 0,
            depth: 0,
        }
    }
}

impl VkSetup for VkExtent3D {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkExtent3D {
    fn vk_free(&self) {
        
    }
}