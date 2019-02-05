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
use vulkan::nvx::{VkObjectEntryType,RawVkObjectEntryType};
use vulkan::nvx::{VkObjectEntryUsageFlags,RawVkObjectEntryUsageFlags};
use vulkan::vk::{VkPipelineLayout,RawVkPipelineLayout};
use vulkan::vk::{VkShaderStageFlags,RawVkShaderStageFlags};

/// Wrapper for [VkObjectTablePushConstantEntryNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkObjectTablePushConstantEntryNVX.html).
#[derive(Debug, Clone)]
pub struct VkObjectTablePushConstantEntry<'a> {
    pub type_: VkObjectEntryType,
    pub flags: VkObjectEntryUsageFlags,
    pub pipeline_layout: &'a VkPipelineLayout,
    pub stage_flags: VkShaderStageFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkObjectTablePushConstantEntry {
    pub type_: RawVkObjectEntryType,
    pub flags: RawVkObjectEntryUsageFlags,
    pub pipeline_layout: RawVkPipelineLayout,
    pub stage_flags: RawVkShaderStageFlags,
}

impl<'a> VkWrappedType<RawVkObjectTablePushConstantEntry> for VkObjectTablePushConstantEntry<'a> {
    fn vk_to_raw(src: &VkObjectTablePushConstantEntry, dst: &mut RawVkObjectTablePushConstantEntry) {
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.pipeline_layout = vk_to_raw_value(src.pipeline_layout);
        dst.stage_flags = vk_to_raw_value(&src.stage_flags);
    }
}

impl Default for VkObjectTablePushConstantEntry<'static> {
    fn default() -> VkObjectTablePushConstantEntry<'static> {
        VkObjectTablePushConstantEntry {
            type_: VkObjectEntryType::default(),
            flags: VkObjectEntryUsageFlags::default(),
            pipeline_layout: vk_null_ref(),
            stage_flags: VkShaderStageFlags::default(),
        }
    }
}

impl<'a> VkSetup for VkObjectTablePushConstantEntry<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkObjectTablePushConstantEntry {
    fn vk_free(&mut self) {
        
    }
}