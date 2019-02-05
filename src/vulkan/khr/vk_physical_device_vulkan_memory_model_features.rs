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

/// Wrapper for [VkPhysicalDeviceVulkanMemoryModelFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeaturesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
    pub vulkan_memory_model: bool,
    pub vulkan_memory_model_device_scope: bool,
    pub vulkan_memory_model_availability_visibility_chains: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub vulkan_memory_model: u32,
    pub vulkan_memory_model_device_scope: u32,
    pub vulkan_memory_model_availability_visibility_chains: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceVulkanMemoryModelFeatures> for VkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceVulkanMemoryModelFeatures, dst: &mut RawVkPhysicalDeviceVulkanMemoryModelFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceVulkanMemoryModelFeaturesKhr);
        dst.next = ptr::null_mut();
        dst.vulkan_memory_model = vk_to_raw_value(&src.vulkan_memory_model);
        dst.vulkan_memory_model_device_scope = vk_to_raw_value(&src.vulkan_memory_model_device_scope);
        dst.vulkan_memory_model_availability_visibility_chains = vk_to_raw_value(&src.vulkan_memory_model_availability_visibility_chains);
    }
}

impl VkRawType<VkPhysicalDeviceVulkanMemoryModelFeatures> for RawVkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceVulkanMemoryModelFeatures) -> VkPhysicalDeviceVulkanMemoryModelFeatures {
        VkPhysicalDeviceVulkanMemoryModelFeatures {
            vulkan_memory_model: u32::vk_to_wrapped(&src.vulkan_memory_model),
            vulkan_memory_model_device_scope: u32::vk_to_wrapped(&src.vulkan_memory_model_device_scope),
            vulkan_memory_model_availability_visibility_chains: u32::vk_to_wrapped(&src.vulkan_memory_model_availability_visibility_chains),
        }
    }
}

impl Default for VkPhysicalDeviceVulkanMemoryModelFeatures {
    fn default() -> VkPhysicalDeviceVulkanMemoryModelFeatures {
        VkPhysicalDeviceVulkanMemoryModelFeatures {
            vulkan_memory_model: false,
            vulkan_memory_model_device_scope: false,
            vulkan_memory_model_availability_visibility_chains: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_free(&self) {
        
    }
}