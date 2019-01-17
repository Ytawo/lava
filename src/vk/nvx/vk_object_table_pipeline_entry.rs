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
use vk::nvx::vk_object_entry_type::*;
use vk::nvx::vk_object_entry_usage_flags::*;
use vk::vk_pipeline::*;

#[derive(Debug, Clone)]
pub struct VkObjectTablePipelineEntry<'a> {
    pub type_: VkObjectEntryType,
    pub flags: VkObjectEntryUsageFlags,
    pub pipeline: &'a VkPipeline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkObjectTablePipelineEntry {
    pub type_: RawVkObjectEntryType,
    pub flags: RawVkObjectEntryUsageFlags,
    pub pipeline: RawVkPipeline,
}

impl<'a> VkWrappedType<RawVkObjectTablePipelineEntry> for VkObjectTablePipelineEntry<'a> {
    fn vk_to_raw(src: &VkObjectTablePipelineEntry, dst: &mut RawVkObjectTablePipelineEntry) {
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.pipeline = vk_to_raw_value(src.pipeline);
    }
}

impl Default for VkObjectTablePipelineEntry<'static> {
    fn default() -> VkObjectTablePipelineEntry<'static> {
        VkObjectTablePipelineEntry {
            type_: VkObjectEntryType::default(),
            flags: VkObjectEntryUsageFlags::default(),
            pipeline: vk_null_ref(),
        }
    }
}

impl<'a> VkSetup for VkObjectTablePipelineEntry<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkObjectTablePipelineEntry {
    fn vk_free(&mut self) {
        
    }
}