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
use vulkan::ext::{VkHeadlessSurfaceCreateFlags,RawVkHeadlessSurfaceCreateFlags};

/// Wrapper for [VkHeadlessSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkHeadlessSurfaceCreateInfo {
    pub flags: VkHeadlessSurfaceCreateFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkHeadlessSurfaceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkHeadlessSurfaceCreateFlags,
}

impl VkWrappedType<RawVkHeadlessSurfaceCreateInfo> for VkHeadlessSurfaceCreateInfo {
    fn vk_to_raw(src: &VkHeadlessSurfaceCreateInfo, dst: &mut RawVkHeadlessSurfaceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::HeadlessSurfaceCreateInfoExt);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl VkRawType<VkHeadlessSurfaceCreateInfo> for RawVkHeadlessSurfaceCreateInfo {
    fn vk_to_wrapped(src: &RawVkHeadlessSurfaceCreateInfo) -> VkHeadlessSurfaceCreateInfo {
        VkHeadlessSurfaceCreateInfo {
            flags: RawVkHeadlessSurfaceCreateFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl Default for VkHeadlessSurfaceCreateInfo {
    fn default() -> VkHeadlessSurfaceCreateInfo {
        VkHeadlessSurfaceCreateInfo {
            flags: Default::default(),
        }
    }
}

impl VkSetup for VkHeadlessSurfaceCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkHeadlessSurfaceCreateInfo {
    fn vk_free(&self) {
        
    }
}