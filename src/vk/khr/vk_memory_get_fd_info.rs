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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::vk_device_memory::*;
use vk::vk_external_memory_handle_type_flags::*;

#[derive(Debug, Clone)]
pub struct VkMemoryGetFdInfo<'a> {
    pub memory: &'a VkDeviceMemory,
    pub handle_type: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryGetFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub memory: RawVkDeviceMemory,
    pub handle_type: RawVkExternalMemoryHandleTypeFlags,
}

impl<'a> VkWrappedType<RawVkMemoryGetFdInfo> for VkMemoryGetFdInfo<'a> {
    fn vk_to_raw(src: &VkMemoryGetFdInfo, dst: &mut RawVkMemoryGetFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryGetFdInfoKhr);
        dst.next = ptr::null();
        dst.memory = vk_to_raw_value(src.memory);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl Default for VkMemoryGetFdInfo<'static> {
    fn default() -> VkMemoryGetFdInfo<'static> {
        VkMemoryGetFdInfo {
            memory: vk_null_ref(),
            handle_type: VkExternalMemoryHandleTypeFlags::default(),
        }
    }
}

impl<'a> VkSetup for VkMemoryGetFdInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMemoryGetFdInfo {
    fn vk_free(&mut self) {
        
    }
}