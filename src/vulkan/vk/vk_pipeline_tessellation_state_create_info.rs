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
use vulkan::vk::{VkPipelineTessellationStateCreateFlags,RawVkPipelineTessellationStateCreateFlags};

/// Wrapper for [VkPipelineTessellationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineTessellationStateCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patch_control_points: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineTessellationStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

impl VkWrappedType<RawVkPipelineTessellationStateCreateInfo> for VkPipelineTessellationStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineTessellationStateCreateInfo, dst: &mut RawVkPipelineTessellationStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineTessellationStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.patch_control_points = vk_to_raw_value(&src.patch_control_points);
    }
}

impl VkRawType<VkPipelineTessellationStateCreateInfo> for RawVkPipelineTessellationStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineTessellationStateCreateInfo) -> VkPipelineTessellationStateCreateInfo {
        VkPipelineTessellationStateCreateInfo {
            flags: RawVkPipelineTessellationStateCreateFlags::vk_to_wrapped(&src.flags),
            patch_control_points: u32::vk_to_wrapped(&src.patch_control_points),
        }
    }
}

impl Default for VkPipelineTessellationStateCreateInfo {
    fn default() -> VkPipelineTessellationStateCreateInfo {
        VkPipelineTessellationStateCreateInfo {
            flags: Default::default(),
            patch_control_points: 0,
        }
    }
}

impl VkSetup for VkPipelineTessellationStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineTessellationStateCreateInfo {
    fn vk_free(&self) {
        
    }
}