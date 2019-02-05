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
use vulkan::vk::{VkRect2D,RawVkRect2D};

/// Wrapper for [VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfo {
    pub exclusive_scissors: Option<Vec<VkRect2D>>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineViewportExclusiveScissorStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub exclusive_scissor_count: u32,
    pub exclusive_scissors: *mut RawVkRect2D,
}

impl VkWrappedType<RawVkPipelineViewportExclusiveScissorStateCreateInfo> for VkPipelineViewportExclusiveScissorStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineViewportExclusiveScissorStateCreateInfo, dst: &mut RawVkPipelineViewportExclusiveScissorStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineViewportExclusiveScissorStateCreateInfoNv);
        dst.next = ptr::null_mut();
        dst.exclusive_scissor_count = get_array_option_len(&src.exclusive_scissors) as u32;
        dst.exclusive_scissors = new_ptr_vk_array_checked(&src.exclusive_scissors);
    }
}

impl VkRawType<VkPipelineViewportExclusiveScissorStateCreateInfo> for RawVkPipelineViewportExclusiveScissorStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineViewportExclusiveScissorStateCreateInfo) -> VkPipelineViewportExclusiveScissorStateCreateInfo {
        VkPipelineViewportExclusiveScissorStateCreateInfo {
            exclusive_scissors: new_vk_array_checked(src.exclusive_scissor_count, src.exclusive_scissors),
        }
    }
}

impl Default for VkPipelineViewportExclusiveScissorStateCreateInfo {
    fn default() -> VkPipelineViewportExclusiveScissorStateCreateInfo {
        VkPipelineViewportExclusiveScissorStateCreateInfo {
            exclusive_scissors: None,
        }
    }
}

impl VkSetup for VkPipelineViewportExclusiveScissorStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineViewportExclusiveScissorStateCreateInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.exclusive_scissor_count as usize, self.exclusive_scissors);
    }
}