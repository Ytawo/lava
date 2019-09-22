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

/// Wrapper for [VkPhysicalDeviceImagelessFramebufferFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeaturesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
    pub imageless_framebuffer: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub imageless_framebuffer: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceImagelessFramebufferFeatures> for VkPhysicalDeviceImagelessFramebufferFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceImagelessFramebufferFeatures, dst: &mut RawVkPhysicalDeviceImagelessFramebufferFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceImagelessFramebufferFeaturesKhr);
        dst.next = ptr::null_mut();
        dst.imageless_framebuffer = vk_to_raw_value(&src.imageless_framebuffer);
    }
}

impl VkRawType<VkPhysicalDeviceImagelessFramebufferFeatures> for RawVkPhysicalDeviceImagelessFramebufferFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceImagelessFramebufferFeatures) -> VkPhysicalDeviceImagelessFramebufferFeatures {
        VkPhysicalDeviceImagelessFramebufferFeatures {
            imageless_framebuffer: u32::vk_to_wrapped(&src.imageless_framebuffer),
        }
    }
}

impl Default for VkPhysicalDeviceImagelessFramebufferFeatures {
    fn default() -> VkPhysicalDeviceImagelessFramebufferFeatures {
        VkPhysicalDeviceImagelessFramebufferFeatures {
            imageless_framebuffer: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceImagelessFramebufferFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceImagelessFramebufferFeatures {
    fn vk_free(&self) {
        
    }
}