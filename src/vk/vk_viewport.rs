// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;

#[derive(Debug, Clone)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl VkWrappedType<RawVkViewport> for VkViewport {
    fn vk_to_raw(src: &VkViewport, dst: &mut RawVkViewport) {
        dst.x = src.x;
        dst.y = src.y;
        dst.width = src.width;
        dst.height = src.height;
        dst.min_depth = src.min_depth;
        dst.max_depth = src.max_depth;
    }
}

impl VkRawType<VkViewport> for RawVkViewport {
    fn vk_to_wrapped(src: &RawVkViewport) -> VkViewport {
        VkViewport {
            x: src.x,
            y: src.y,
            width: src.width,
            height: src.height,
            min_depth: src.min_depth,
            max_depth: src.max_depth,
        }
    }
}

impl Default for VkViewport {
    fn default() -> VkViewport {
        VkViewport {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 0.0,
        }
    }
}

impl VkSetup for VkViewport {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkViewport {
    fn vk_free(&mut self) {
        
    }
}