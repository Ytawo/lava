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

/// Wrapper for [VkBaseOutStructure](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBaseOutStructure.html).
#[derive(Debug, Clone)]
pub struct VkBaseOutStructure {
    pub s_type: VkStructureType,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBaseOutStructure {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
}

impl VkWrappedType<RawVkBaseOutStructure> for VkBaseOutStructure {
    fn vk_to_raw(src: &VkBaseOutStructure, dst: &mut RawVkBaseOutStructure) {
        dst.s_type = vk_to_raw_value(&src.s_type);
        dst.next = ptr::null_mut();
    }
}

impl VkRawType<VkBaseOutStructure> for RawVkBaseOutStructure {
    fn vk_to_wrapped(src: &RawVkBaseOutStructure) -> VkBaseOutStructure {
        VkBaseOutStructure {
            s_type: RawVkStructureType::vk_to_wrapped(&src.s_type),
        }
    }
}

impl Default for VkBaseOutStructure {
    fn default() -> VkBaseOutStructure {
        VkBaseOutStructure {
            s_type: Default::default(),
        }
    }
}

impl VkSetup for VkBaseOutStructure {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkBaseOutStructure {
    fn vk_free(&self) {
        
    }
}