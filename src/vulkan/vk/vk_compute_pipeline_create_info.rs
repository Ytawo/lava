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
use vulkan::vk::{VkPipelineCreateFlags,RawVkPipelineCreateFlags};
use vulkan::vk::{VkPipelineShaderStageCreateInfo,RawVkPipelineShaderStageCreateInfo};
use vulkan::vk::{VkPipelineLayout,RawVkPipelineLayout};
use vulkan::vk::{VkPipeline,RawVkPipeline};

/// Wrapper for [VkComputePipelineCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkComputePipelineCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkComputePipelineCreateInfo<'a, 'b> {
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo<'a, 'b>,
    pub layout: VkPipelineLayout,
    pub base_pipeline_handle: Option<VkPipeline>,
    pub base_pipeline_index: isize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkComputePipelineCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkPipelineCreateFlags,
    pub stage: RawVkPipelineShaderStageCreateInfo,
    pub layout: RawVkPipelineLayout,
    pub base_pipeline_handle: RawVkPipeline,
    pub base_pipeline_index: i32,
}

impl<'a, 'b> VkWrappedType<RawVkComputePipelineCreateInfo> for VkComputePipelineCreateInfo<'a, 'b> {
    fn vk_to_raw(src: &VkComputePipelineCreateInfo, dst: &mut RawVkComputePipelineCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ComputePipelineCreateInfo);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.stage = vk_to_raw_value(&src.stage);
        dst.layout = vk_to_raw_value(&src.layout);
        dst.base_pipeline_handle = vk_to_raw_value_checked(&src.base_pipeline_handle);
        dst.base_pipeline_index = vk_to_raw_value(&src.base_pipeline_index);
    }
}

impl<'a, 'b> VkRawType<VkComputePipelineCreateInfo<'a, 'b>> for RawVkComputePipelineCreateInfo {
    fn vk_to_wrapped(src: &RawVkComputePipelineCreateInfo) -> VkComputePipelineCreateInfo<'a, 'b> {
        VkComputePipelineCreateInfo {
            flags: RawVkPipelineCreateFlags::vk_to_wrapped(&src.flags),
            stage: RawVkPipelineShaderStageCreateInfo::vk_to_wrapped(&src.stage),
            layout: RawVkPipelineLayout::vk_to_wrapped(&src.layout),
            base_pipeline_handle: Some(RawVkPipeline::vk_to_wrapped(&src.base_pipeline_handle)),
            base_pipeline_index: i32::vk_to_wrapped(&src.base_pipeline_index),
        }
    }
}

impl Default for VkComputePipelineCreateInfo<'static, 'static> {
    fn default() -> VkComputePipelineCreateInfo<'static, 'static> {
        VkComputePipelineCreateInfo {
            flags: Default::default(),
            stage: Default::default(),
            layout: Default::default(),
            base_pipeline_handle: None,
            base_pipeline_index: 0,
        }
    }
}

impl<'a, 'b> VkSetup for VkComputePipelineCreateInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.layout, fn_table);
    }
}

impl VkFree for RawVkComputePipelineCreateInfo {
    fn vk_free(&self) {
        
    }
}