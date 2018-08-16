// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::vk_shader_stage_flags::*;
use vk::vk_subgroup_feature_flags::*;

#[repr(C)]
pub struct RawVkPhysicalDeviceSubgroupProperties {
    s_type: RawVkStructureType,
    next: *const c_void,
    subgroup_size: u32,
    supported_stages: RawVkShaderStageFlags,
    supported_operations: RawVkSubgroupFeatureFlags,
    quad_operations_in_all_stages: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSubgroupProperties {
    pub subgroup_size: usize,
    pub supported_stages: VkShaderStageFlags,
    pub supported_operations: VkSubgroupFeatureFlags,
    pub quad_operations_in_all_stages: bool,
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

impl VkWrappedType<RawVkPhysicalDeviceSubgroupProperties> for VkPhysicalDeviceSubgroupProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceSubgroupProperties, dst: &mut RawVkPhysicalDeviceSubgroupProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSubgroupProperties);
        dst.next = ptr::null();
        dst.subgroup_size = vk_to_raw_value(&src.subgroup_size);
        dst.supported_stages = vk_to_raw_value(&src.supported_stages);
        dst.supported_operations = vk_to_raw_value(&src.supported_operations);
        dst.quad_operations_in_all_stages = vk_to_raw_value(&src.quad_operations_in_all_stages);
    }
}

impl Default for VkPhysicalDeviceSubgroupProperties {
    fn default() -> VkPhysicalDeviceSubgroupProperties {
        VkPhysicalDeviceSubgroupProperties {
            subgroup_size: 0,
            supported_stages: VkShaderStageFlags::default(),
            supported_operations: VkSubgroupFeatureFlags::default(),
            quad_operations_in_all_stages: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceSubgroupProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceSubgroupProperties {
    fn vk_free(&mut self) {
        
    }
}