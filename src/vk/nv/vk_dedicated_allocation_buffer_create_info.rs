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

#[repr(C)]
pub struct RawVkDedicatedAllocationBufferCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    dedicated_allocation: u32,
}

#[derive(Debug, Clone)]
pub struct VkDedicatedAllocationBufferCreateInfo {
    pub dedicated_allocation: bool,
}

impl VkRawType<VkDedicatedAllocationBufferCreateInfo> for RawVkDedicatedAllocationBufferCreateInfo {
    fn vk_to_wrapped(src: &RawVkDedicatedAllocationBufferCreateInfo) -> VkDedicatedAllocationBufferCreateInfo {
        VkDedicatedAllocationBufferCreateInfo {
            dedicated_allocation: u32::vk_to_wrapped(&src.dedicated_allocation),
        }
    }
}

impl VkWrappedType<RawVkDedicatedAllocationBufferCreateInfo> for VkDedicatedAllocationBufferCreateInfo {
    fn vk_to_raw(src: &VkDedicatedAllocationBufferCreateInfo, dst: &mut RawVkDedicatedAllocationBufferCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DedicatedAllocationBufferCreateInfoNv);
        dst.next = ptr::null();
        dst.dedicated_allocation = vk_to_raw_value(&src.dedicated_allocation);
    }
}

impl Default for VkDedicatedAllocationBufferCreateInfo {
    fn default() -> VkDedicatedAllocationBufferCreateInfo {
        VkDedicatedAllocationBufferCreateInfo {
            dedicated_allocation: false,
        }
    }
}

impl VkSetup for VkDedicatedAllocationBufferCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDedicatedAllocationBufferCreateInfo {
    fn vk_free(&mut self) {
        
    }
}