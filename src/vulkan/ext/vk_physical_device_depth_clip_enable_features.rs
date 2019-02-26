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

/// Wrapper for [VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDepthClipEnableFeatures {
    pub depth_clip_enable: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDepthClipEnableFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub depth_clip_enable: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDepthClipEnableFeatures> for VkPhysicalDeviceDepthClipEnableFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceDepthClipEnableFeatures, dst: &mut RawVkPhysicalDeviceDepthClipEnableFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDepthClipEnableFeaturesExt);
        dst.next = ptr::null_mut();
        dst.depth_clip_enable = vk_to_raw_value(&src.depth_clip_enable);
    }
}

impl VkRawType<VkPhysicalDeviceDepthClipEnableFeatures> for RawVkPhysicalDeviceDepthClipEnableFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDepthClipEnableFeatures) -> VkPhysicalDeviceDepthClipEnableFeatures {
        VkPhysicalDeviceDepthClipEnableFeatures {
            depth_clip_enable: u32::vk_to_wrapped(&src.depth_clip_enable),
        }
    }
}

impl Default for VkPhysicalDeviceDepthClipEnableFeatures {
    fn default() -> VkPhysicalDeviceDepthClipEnableFeatures {
        VkPhysicalDeviceDepthClipEnableFeatures {
            depth_clip_enable: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDepthClipEnableFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDepthClipEnableFeatures {
    fn vk_free(&self) {
        
    }
}