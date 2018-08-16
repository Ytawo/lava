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
use vk::vk_semaphore_create_flags::*;

#[repr(C)]
pub struct RawVkSemaphoreCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkSemaphoreCreateFlags,
}

#[derive(Debug, Clone)]
pub struct VkSemaphoreCreateInfo {
    pub flags: VkSemaphoreCreateFlags,
}

impl VkRawType<VkSemaphoreCreateInfo> for RawVkSemaphoreCreateInfo {
    fn vk_to_wrapped(src: &RawVkSemaphoreCreateInfo) -> VkSemaphoreCreateInfo {
        VkSemaphoreCreateInfo {
            flags: RawVkSemaphoreCreateFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl VkWrappedType<RawVkSemaphoreCreateInfo> for VkSemaphoreCreateInfo {
    fn vk_to_raw(src: &VkSemaphoreCreateInfo, dst: &mut RawVkSemaphoreCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SemaphoreCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl Default for VkSemaphoreCreateInfo {
    fn default() -> VkSemaphoreCreateInfo {
        VkSemaphoreCreateInfo {
            flags: VkSemaphoreCreateFlags::default(),
        }
    }
}

impl VkSetup for VkSemaphoreCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSemaphoreCreateInfo {
    fn vk_free(&mut self) {
        
    }
}