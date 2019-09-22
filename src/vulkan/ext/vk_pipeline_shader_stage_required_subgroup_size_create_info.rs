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

/// Wrapper for [VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub required_subgroup_size: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub required_subgroup_size: u32,
}

impl VkWrappedType<RawVkPipelineShaderStageRequiredSubgroupSizeCreateInfo> for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn vk_to_raw(src: &VkPipelineShaderStageRequiredSubgroupSizeCreateInfo, dst: &mut RawVkPipelineShaderStageRequiredSubgroupSizeCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineShaderStageRequiredSubgroupSizeCreateInfoExt);
        dst.next = ptr::null_mut();
        dst.required_subgroup_size = vk_to_raw_value(&src.required_subgroup_size);
    }
}

impl VkRawType<VkPipelineShaderStageRequiredSubgroupSizeCreateInfo> for RawVkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineShaderStageRequiredSubgroupSizeCreateInfo) -> VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
        VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
            required_subgroup_size: u32::vk_to_wrapped(&src.required_subgroup_size),
        }
    }
}

impl Default for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn default() -> VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
        VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
            required_subgroup_size: 0,
        }
    }
}

impl VkSetup for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn vk_free(&self) {
        
    }
}