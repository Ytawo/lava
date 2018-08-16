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
use vk::vk_buffer::*;
use vk::vk_index_type::*;

#[repr(C)]
pub struct RawVkObjectTableIndexBufferEntry {
    type_: RawVkObjectEntryType,
    flags: RawVkObjectEntryUsageFlags,
    buffer: RawVkBuffer,
    index_type: RawVkIndexType,
}

#[derive(Debug, Clone)]
pub struct VkObjectTableIndexBufferEntry<'a> {
    pub type_: VkObjectEntryType,
    pub flags: VkObjectEntryUsageFlags,
    pub buffer: &'a VkBuffer,
    pub index_type: VkIndexType,
}

impl<'a> VkWrappedType<RawVkObjectTableIndexBufferEntry> for VkObjectTableIndexBufferEntry<'a> {
    fn vk_to_raw(src: &VkObjectTableIndexBufferEntry, dst: &mut RawVkObjectTableIndexBufferEntry) {
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.buffer = vk_to_raw_value(src.buffer);
        dst.index_type = vk_to_raw_value(&src.index_type);
    }
}

impl Default for VkObjectTableIndexBufferEntry<'static> {
    fn default() -> VkObjectTableIndexBufferEntry<'static> {
        VkObjectTableIndexBufferEntry {
            type_: VkObjectEntryType::default(),
            flags: VkObjectEntryUsageFlags::default(),
            buffer: vk_null_ref(),
            index_type: VkIndexType::default(),
        }
    }
}

impl<'a> VkSetup for VkObjectTableIndexBufferEntry<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkObjectTableIndexBufferEntry {
    fn vk_free(&mut self) {
        
    }
}