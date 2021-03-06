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
use vulkan::vk::{VkPipelineDynamicStateCreateFlags,RawVkPipelineDynamicStateCreateFlags};
use vulkan::vk::{VkDynamicState,RawVkDynamicState};

/// Wrapper for [VkPipelineDynamicStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamic_states: Vec<VkDynamicState>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineDynamicStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkPipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub dynamic_states: *mut RawVkDynamicState,
}

impl VkWrappedType<RawVkPipelineDynamicStateCreateInfo> for VkPipelineDynamicStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineDynamicStateCreateInfo, dst: &mut RawVkPipelineDynamicStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineDynamicStateCreateInfo);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.dynamic_state_count = src.dynamic_states.len() as u32;
        dst.dynamic_states = new_ptr_vk_array(&src.dynamic_states);
    }
}

impl VkRawType<VkPipelineDynamicStateCreateInfo> for RawVkPipelineDynamicStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineDynamicStateCreateInfo) -> VkPipelineDynamicStateCreateInfo {
        VkPipelineDynamicStateCreateInfo {
            flags: RawVkPipelineDynamicStateCreateFlags::vk_to_wrapped(&src.flags),
            dynamic_states: new_vk_array(src.dynamic_state_count, src.dynamic_states),
        }
    }
}

impl Default for VkPipelineDynamicStateCreateInfo {
    fn default() -> VkPipelineDynamicStateCreateInfo {
        VkPipelineDynamicStateCreateInfo {
            flags: Default::default(),
            dynamic_states: Vec::new(),
        }
    }
}

impl VkSetup for VkPipelineDynamicStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineDynamicStateCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.dynamic_states);
    }
}