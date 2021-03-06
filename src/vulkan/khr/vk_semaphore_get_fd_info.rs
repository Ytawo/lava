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
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::vk::{VkSemaphore,RawVkSemaphore};
use vulkan::vk::{VkExternalSemaphoreHandleTypeFlags,RawVkExternalSemaphoreHandleTypeFlags};

/// Wrapper for [VkSemaphoreGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetFdInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkSemaphoreGetFdInfo {
    pub semaphore: VkSemaphore,
    pub handle_type: VkExternalSemaphoreHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSemaphoreGetFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub semaphore: RawVkSemaphore,
    pub handle_type: RawVkExternalSemaphoreHandleTypeFlags,
}

impl VkWrappedType<RawVkSemaphoreGetFdInfo> for VkSemaphoreGetFdInfo {
    fn vk_to_raw(src: &VkSemaphoreGetFdInfo, dst: &mut RawVkSemaphoreGetFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SemaphoreGetFdInfoKhr);
        dst.next = ptr::null_mut();
        dst.semaphore = vk_to_raw_value(&src.semaphore);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl VkRawType<VkSemaphoreGetFdInfo> for RawVkSemaphoreGetFdInfo {
    fn vk_to_wrapped(src: &RawVkSemaphoreGetFdInfo) -> VkSemaphoreGetFdInfo {
        VkSemaphoreGetFdInfo {
            semaphore: RawVkSemaphore::vk_to_wrapped(&src.semaphore),
            handle_type: RawVkExternalSemaphoreHandleTypeFlags::vk_to_wrapped(&src.handle_type),
        }
    }
}

impl Default for VkSemaphoreGetFdInfo {
    fn default() -> VkSemaphoreGetFdInfo {
        VkSemaphoreGetFdInfo {
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}

impl VkSetup for VkSemaphoreGetFdInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.semaphore, fn_table);
    }
}

impl VkFree for RawVkSemaphoreGetFdInfo {
    fn vk_free(&self) {
        
    }
}