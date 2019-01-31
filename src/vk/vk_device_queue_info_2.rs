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
use vk::vk_device_queue_create_flags::*;

#[derive(Debug, Clone)]
pub struct VkDeviceQueueInfo2 {
    pub flags: VkDeviceQueueCreateFlags,
    pub queue_family_index: usize,
    pub queue_index: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceQueueInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

impl VkWrappedType<RawVkDeviceQueueInfo2> for VkDeviceQueueInfo2 {
    fn vk_to_raw(src: &VkDeviceQueueInfo2, dst: &mut RawVkDeviceQueueInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceQueueInfo2);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.queue_family_index = vk_to_raw_value(&src.queue_family_index);
        dst.queue_index = vk_to_raw_value(&src.queue_index);
    }
}

impl VkRawType<VkDeviceQueueInfo2> for RawVkDeviceQueueInfo2 {
    fn vk_to_wrapped(src: &RawVkDeviceQueueInfo2) -> VkDeviceQueueInfo2 {
        VkDeviceQueueInfo2 {
            flags: RawVkDeviceQueueCreateFlags::vk_to_wrapped(&src.flags),
            queue_family_index: u32::vk_to_wrapped(&src.queue_family_index),
            queue_index: u32::vk_to_wrapped(&src.queue_index),
        }
    }
}

impl Default for VkDeviceQueueInfo2 {
    fn default() -> VkDeviceQueueInfo2 {
        VkDeviceQueueInfo2 {
            flags: VkDeviceQueueCreateFlags::default(),
            queue_family_index: 0,
            queue_index: 0,
        }
    }
}

impl VkSetup for VkDeviceQueueInfo2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceQueueInfo2 {
    fn vk_free(&mut self) {
        
    }
}