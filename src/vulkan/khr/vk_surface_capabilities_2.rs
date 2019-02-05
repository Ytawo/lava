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
use vulkan::khr::{VkSurfaceCapabilities,RawVkSurfaceCapabilities};

/// Wrapper for [VkSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSurfaceCapabilities2KHR.html).
#[derive(Debug, Clone)]
pub struct VkSurfaceCapabilities2 {
    pub surface_capabilities: VkSurfaceCapabilities,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSurfaceCapabilities2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub surface_capabilities: RawVkSurfaceCapabilities,
}

impl VkWrappedType<RawVkSurfaceCapabilities2> for VkSurfaceCapabilities2 {
    fn vk_to_raw(src: &VkSurfaceCapabilities2, dst: &mut RawVkSurfaceCapabilities2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SurfaceCapabilities2Khr);
        dst.next = ptr::null();
        dst.surface_capabilities = vk_to_raw_value(&src.surface_capabilities);
    }
}

impl VkRawType<VkSurfaceCapabilities2> for RawVkSurfaceCapabilities2 {
    fn vk_to_wrapped(src: &RawVkSurfaceCapabilities2) -> VkSurfaceCapabilities2 {
        VkSurfaceCapabilities2 {
            surface_capabilities: RawVkSurfaceCapabilities::vk_to_wrapped(&src.surface_capabilities),
        }
    }
}

impl Default for VkSurfaceCapabilities2 {
    fn default() -> VkSurfaceCapabilities2 {
        VkSurfaceCapabilities2 {
            surface_capabilities: Default::default(),
        }
    }
}

impl VkSetup for VkSurfaceCapabilities2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.surface_capabilities, fn_table);
    }
}

impl VkFree for RawVkSurfaceCapabilities2 {
    fn vk_free(&self) {
        
    }
}