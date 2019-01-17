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
use vk::vk_buffer_create_flags::*;
use vk::vk_buffer_usage_flags::*;
use vk::vk_external_memory_handle_type_flags::*;

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfo {
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handle_type: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceExternalBufferInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkBufferCreateFlags,
    pub usage: RawVkBufferUsageFlags,
    pub handle_type: RawVkExternalMemoryHandleTypeFlags,
}

impl VkWrappedType<RawVkPhysicalDeviceExternalBufferInfo> for VkPhysicalDeviceExternalBufferInfo {
    fn vk_to_raw(src: &VkPhysicalDeviceExternalBufferInfo, dst: &mut RawVkPhysicalDeviceExternalBufferInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceExternalBufferInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.usage = vk_to_raw_value(&src.usage);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl VkRawType<VkPhysicalDeviceExternalBufferInfo> for RawVkPhysicalDeviceExternalBufferInfo {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceExternalBufferInfo) -> VkPhysicalDeviceExternalBufferInfo {
        VkPhysicalDeviceExternalBufferInfo {
            flags: RawVkBufferCreateFlags::vk_to_wrapped(&src.flags),
            usage: RawVkBufferUsageFlags::vk_to_wrapped(&src.usage),
            handle_type: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.handle_type),
        }
    }
}

impl Default for VkPhysicalDeviceExternalBufferInfo {
    fn default() -> VkPhysicalDeviceExternalBufferInfo {
        VkPhysicalDeviceExternalBufferInfo {
            flags: VkBufferCreateFlags::default(),
            usage: VkBufferUsageFlags::default(),
            handle_type: VkExternalMemoryHandleTypeFlags::default(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceExternalBufferInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceExternalBufferInfo {
    fn vk_free(&mut self) {
        
    }
}