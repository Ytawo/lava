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

/// Wrapper for [VkSurfaceProtectedCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkSurfaceProtectedCapabilities {
    pub supports_protected: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSurfaceProtectedCapabilities {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub supports_protected: u32,
}

impl VkWrappedType<RawVkSurfaceProtectedCapabilities> for VkSurfaceProtectedCapabilities {
    fn vk_to_raw(src: &VkSurfaceProtectedCapabilities, dst: &mut RawVkSurfaceProtectedCapabilities) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SurfaceProtectedCapabilitiesKhr);
        dst.next = ptr::null_mut();
        dst.supports_protected = vk_to_raw_value(&src.supports_protected);
    }
}

impl VkRawType<VkSurfaceProtectedCapabilities> for RawVkSurfaceProtectedCapabilities {
    fn vk_to_wrapped(src: &RawVkSurfaceProtectedCapabilities) -> VkSurfaceProtectedCapabilities {
        VkSurfaceProtectedCapabilities {
            supports_protected: u32::vk_to_wrapped(&src.supports_protected),
        }
    }
}

impl Default for VkSurfaceProtectedCapabilities {
    fn default() -> VkSurfaceProtectedCapabilities {
        VkSurfaceProtectedCapabilities {
            supports_protected: false,
        }
    }
}

impl VkSetup for VkSurfaceProtectedCapabilities {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSurfaceProtectedCapabilities {
    fn vk_free(&self) {
        
    }
}