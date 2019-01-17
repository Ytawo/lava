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
pub struct VkDeviceGroupSubmitInfo<'a, 'b, 'c> {
    pub wait_semaphore_device_indices: &'a [usize],
    pub command_buffer_device_masks: &'b [u32],
    pub signal_semaphore_device_indices: &'c [usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupSubmitInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub wait_semaphore_count: u32,
    pub wait_semaphore_device_indices: *mut u32,
    pub command_buffer_count: u32,
    pub command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub signal_semaphore_device_indices: *mut u32,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkDeviceGroupSubmitInfo> for VkDeviceGroupSubmitInfo<'a, 'b, 'c> {
    fn vk_to_raw(src: &VkDeviceGroupSubmitInfo, dst: &mut RawVkDeviceGroupSubmitInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupSubmitInfo);
        dst.next = ptr::null();
        dst.wait_semaphore_count = src.wait_semaphore_device_indices.len() as u32;
        dst.wait_semaphore_device_indices = new_ptr_vk_array(src.wait_semaphore_device_indices);
        dst.command_buffer_count = src.command_buffer_device_masks.len() as u32;
        dst.command_buffer_device_masks = src.command_buffer_device_masks.as_ptr();
        dst.signal_semaphore_count = src.signal_semaphore_device_indices.len() as u32;
        dst.signal_semaphore_device_indices = new_ptr_vk_array(src.signal_semaphore_device_indices);
    }
}

impl Default for VkDeviceGroupSubmitInfo<'static, 'static, 'static> {
    fn default() -> VkDeviceGroupSubmitInfo<'static, 'static, 'static> {
        VkDeviceGroupSubmitInfo {
            wait_semaphore_device_indices: &[],
            command_buffer_device_masks: &[],
            signal_semaphore_device_indices: &[],
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkDeviceGroupSubmitInfo<'a, 'b, 'c> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGroupSubmitInfo {
    fn vk_free(&mut self) {
        free_ptr(self.wait_semaphore_device_indices);
        free_ptr(self.signal_semaphore_device_indices);
    }
}