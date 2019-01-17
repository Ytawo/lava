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
use vk::vk_structure_type::*;

#[derive(Debug, Clone)]
pub struct VkBindBufferMemoryDeviceGroupInfo<'a> {
    pub device_indices: &'a [usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBindBufferMemoryDeviceGroupInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub device_index_count: u32,
    pub device_indices: *mut u32,
}

impl<'a> VkWrappedType<RawVkBindBufferMemoryDeviceGroupInfo> for VkBindBufferMemoryDeviceGroupInfo<'a> {
    fn vk_to_raw(src: &VkBindBufferMemoryDeviceGroupInfo, dst: &mut RawVkBindBufferMemoryDeviceGroupInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindBufferMemoryDeviceGroupInfo);
        dst.next = ptr::null();
        dst.device_index_count = src.device_indices.len() as u32;
        dst.device_indices = new_ptr_vk_array(src.device_indices);
    }
}

impl Default for VkBindBufferMemoryDeviceGroupInfo<'static> {
    fn default() -> VkBindBufferMemoryDeviceGroupInfo<'static> {
        VkBindBufferMemoryDeviceGroupInfo {
            device_indices: &[],
        }
    }
}

impl<'a> VkSetup for VkBindBufferMemoryDeviceGroupInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBindBufferMemoryDeviceGroupInfo {
    fn vk_free(&mut self) {
        free_ptr(self.device_indices);
    }
}