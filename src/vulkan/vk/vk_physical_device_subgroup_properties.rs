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
use vulkan::vk::{VkShaderStageFlags,RawVkShaderStageFlags};
use vulkan::vk::{VkSubgroupFeatureFlags,RawVkSubgroupFeatureFlags};

/// Wrapper for [VkPhysicalDeviceSubgroupProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSubgroupProperties {
    pub subgroup_size: usize,
    pub supported_stages: VkShaderStageFlags,
    pub supported_operations: VkSubgroupFeatureFlags,
    pub quad_operations_in_all_stages: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceSubgroupProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub subgroup_size: u32,
    pub supported_stages: RawVkShaderStageFlags,
    pub supported_operations: RawVkSubgroupFeatureFlags,
    pub quad_operations_in_all_stages: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceSubgroupProperties> for VkPhysicalDeviceSubgroupProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceSubgroupProperties, dst: &mut RawVkPhysicalDeviceSubgroupProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSubgroupProperties);
        dst.next = ptr::null_mut();
        dst.subgroup_size = vk_to_raw_value(&src.subgroup_size);
        dst.supported_stages = vk_to_raw_value(&src.supported_stages);
        dst.supported_operations = vk_to_raw_value(&src.supported_operations);
        dst.quad_operations_in_all_stages = vk_to_raw_value(&src.quad_operations_in_all_stages);
    }
}

impl VkRawType<VkPhysicalDeviceSubgroupProperties> for RawVkPhysicalDeviceSubgroupProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceSubgroupProperties) -> VkPhysicalDeviceSubgroupProperties {
        VkPhysicalDeviceSubgroupProperties {
            subgroup_size: u32::vk_to_wrapped(&src.subgroup_size),
            supported_stages: RawVkShaderStageFlags::vk_to_wrapped(&src.supported_stages),
            supported_operations: RawVkSubgroupFeatureFlags::vk_to_wrapped(&src.supported_operations),
            quad_operations_in_all_stages: u32::vk_to_wrapped(&src.quad_operations_in_all_stages),
        }
    }
}

impl Default for VkPhysicalDeviceSubgroupProperties {
    fn default() -> VkPhysicalDeviceSubgroupProperties {
        VkPhysicalDeviceSubgroupProperties {
            subgroup_size: 0,
            supported_stages: Default::default(),
            supported_operations: Default::default(),
            quad_operations_in_all_stages: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceSubgroupProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceSubgroupProperties {
    fn vk_free(&self) {
        
    }
}