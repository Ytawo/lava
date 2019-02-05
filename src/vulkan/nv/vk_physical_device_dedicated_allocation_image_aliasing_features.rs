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

/// Wrapper for [VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    pub dedicated_allocation_image_aliasing: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub dedicated_allocation_image_aliasing: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDedicatedAllocationImageAliasingFeatures> for VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures, dst: &mut RawVkPhysicalDeviceDedicatedAllocationImageAliasingFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv);
        dst.next = ptr::null_mut();
        dst.dedicated_allocation_image_aliasing = vk_to_raw_value(&src.dedicated_allocation_image_aliasing);
    }
}

impl VkRawType<VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures> for RawVkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDedicatedAllocationImageAliasingFeatures) -> VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
        VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
            dedicated_allocation_image_aliasing: u32::vk_to_wrapped(&src.dedicated_allocation_image_aliasing),
        }
    }
}

impl Default for VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    fn default() -> VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
        VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
            dedicated_allocation_image_aliasing: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDedicatedAllocationImageAliasingFeatures {
    fn vk_free(&self) {
        
    }
}