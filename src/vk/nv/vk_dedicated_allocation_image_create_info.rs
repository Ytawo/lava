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
#[derive(Debug, Copy, Clone)]
pub struct RawVkDedicatedAllocationImageCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub dedicated_allocation: u32,
}

#[derive(Debug, Clone)]
pub struct VkDedicatedAllocationImageCreateInfo {
    pub dedicated_allocation: bool,
}

impl VkRawType<VkDedicatedAllocationImageCreateInfo> for RawVkDedicatedAllocationImageCreateInfo {
    fn vk_to_wrapped(src: &RawVkDedicatedAllocationImageCreateInfo) -> VkDedicatedAllocationImageCreateInfo {
        VkDedicatedAllocationImageCreateInfo {
            dedicated_allocation: u32::vk_to_wrapped(&src.dedicated_allocation),
        }
    }
}

impl VkWrappedType<RawVkDedicatedAllocationImageCreateInfo> for VkDedicatedAllocationImageCreateInfo {
    fn vk_to_raw(src: &VkDedicatedAllocationImageCreateInfo, dst: &mut RawVkDedicatedAllocationImageCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DedicatedAllocationImageCreateInfoNv);
        dst.next = ptr::null();
        dst.dedicated_allocation = vk_to_raw_value(&src.dedicated_allocation);
    }
}

impl Default for VkDedicatedAllocationImageCreateInfo {
    fn default() -> VkDedicatedAllocationImageCreateInfo {
        VkDedicatedAllocationImageCreateInfo {
            dedicated_allocation: false,
        }
    }
}

impl VkSetup for VkDedicatedAllocationImageCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDedicatedAllocationImageCreateInfo {
    fn vk_free(&mut self) {
        
    }
}