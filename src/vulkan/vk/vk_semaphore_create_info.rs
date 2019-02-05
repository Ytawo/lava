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
use vulkan::vk::{VkSemaphoreCreateFlags,RawVkSemaphoreCreateFlags};

/// Wrapper for [VkSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSemaphoreCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkSemaphoreCreateInfo {
    pub flags: VkSemaphoreCreateFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSemaphoreCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkSemaphoreCreateFlags,
}

impl VkWrappedType<RawVkSemaphoreCreateInfo> for VkSemaphoreCreateInfo {
    fn vk_to_raw(src: &VkSemaphoreCreateInfo, dst: &mut RawVkSemaphoreCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SemaphoreCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl VkRawType<VkSemaphoreCreateInfo> for RawVkSemaphoreCreateInfo {
    fn vk_to_wrapped(src: &RawVkSemaphoreCreateInfo) -> VkSemaphoreCreateInfo {
        VkSemaphoreCreateInfo {
            flags: RawVkSemaphoreCreateFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl Default for VkSemaphoreCreateInfo {
    fn default() -> VkSemaphoreCreateInfo {
        VkSemaphoreCreateInfo {
            flags: Default::default(),
        }
    }
}

impl VkSetup for VkSemaphoreCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSemaphoreCreateInfo {
    fn vk_free(&self) {
        
    }
}