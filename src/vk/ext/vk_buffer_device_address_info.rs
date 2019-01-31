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
use vk::vk_buffer::*;

#[derive(Debug, Clone)]
pub struct VkBufferDeviceAddressInfo<'a> {
    pub buffer: &'a VkBuffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferDeviceAddressInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub buffer: RawVkBuffer,
}

impl<'a> VkWrappedType<RawVkBufferDeviceAddressInfo> for VkBufferDeviceAddressInfo<'a> {
    fn vk_to_raw(src: &VkBufferDeviceAddressInfo, dst: &mut RawVkBufferDeviceAddressInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferDeviceAddressInfoExt);
        dst.next = ptr::null();
        dst.buffer = vk_to_raw_value(src.buffer);
    }
}

impl Default for VkBufferDeviceAddressInfo<'static> {
    fn default() -> VkBufferDeviceAddressInfo<'static> {
        VkBufferDeviceAddressInfo {
            buffer: vk_null_ref(),
        }
    }
}

impl<'a> VkSetup for VkBufferDeviceAddressInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBufferDeviceAddressInfo {
    fn vk_free(&mut self) {
        
    }
}