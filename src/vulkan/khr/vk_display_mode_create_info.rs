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
use vulkan::khr::{VkDisplayModeCreateFlags,RawVkDisplayModeCreateFlags};
use vulkan::khr::{VkDisplayModeParameters,RawVkDisplayModeParameters};

/// Wrapper for [VkDisplayModeCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayModeCreateInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkDisplayModeCreateInfo {
    pub flags: VkDisplayModeCreateFlags,
    pub parameters: VkDisplayModeParameters,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayModeCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDisplayModeCreateFlags,
    pub parameters: RawVkDisplayModeParameters,
}

impl VkWrappedType<RawVkDisplayModeCreateInfo> for VkDisplayModeCreateInfo {
    fn vk_to_raw(src: &VkDisplayModeCreateInfo, dst: &mut RawVkDisplayModeCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayModeCreateInfoKhr);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.parameters = vk_to_raw_value(&src.parameters);
    }
}

impl VkRawType<VkDisplayModeCreateInfo> for RawVkDisplayModeCreateInfo {
    fn vk_to_wrapped(src: &RawVkDisplayModeCreateInfo) -> VkDisplayModeCreateInfo {
        VkDisplayModeCreateInfo {
            flags: RawVkDisplayModeCreateFlags::vk_to_wrapped(&src.flags),
            parameters: RawVkDisplayModeParameters::vk_to_wrapped(&src.parameters),
        }
    }
}

impl Default for VkDisplayModeCreateInfo {
    fn default() -> VkDisplayModeCreateInfo {
        VkDisplayModeCreateInfo {
            flags: VkDisplayModeCreateFlags::default(),
            parameters: VkDisplayModeParameters::default(),
        }
    }
}

impl VkSetup for VkDisplayModeCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.parameters, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayModeCreateInfo {
    fn vk_free(&mut self) {
        RawVkDisplayModeParameters::vk_free(&mut self.parameters);
    }
}