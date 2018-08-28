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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub filter_minmax_single_component_formats: u32,
    pub filter_minmax_image_component_mapping: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
    pub filter_minmax_single_component_formats: bool,
    pub filter_minmax_image_component_mapping: bool,
}

impl VkRawType<VkPhysicalDeviceSamplerFilterMinmaxProperties> for RawVkPhysicalDeviceSamplerFilterMinmaxProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceSamplerFilterMinmaxProperties) -> VkPhysicalDeviceSamplerFilterMinmaxProperties {
        VkPhysicalDeviceSamplerFilterMinmaxProperties {
            filter_minmax_single_component_formats: u32::vk_to_wrapped(&src.filter_minmax_single_component_formats),
            filter_minmax_image_component_mapping: u32::vk_to_wrapped(&src.filter_minmax_image_component_mapping),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceSamplerFilterMinmaxProperties> for VkPhysicalDeviceSamplerFilterMinmaxProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceSamplerFilterMinmaxProperties, dst: &mut RawVkPhysicalDeviceSamplerFilterMinmaxProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSamplerFilterMinmaxPropertiesExt);
        dst.next = ptr::null();
        dst.filter_minmax_single_component_formats = vk_to_raw_value(&src.filter_minmax_single_component_formats);
        dst.filter_minmax_image_component_mapping = vk_to_raw_value(&src.filter_minmax_image_component_mapping);
    }
}

impl Default for VkPhysicalDeviceSamplerFilterMinmaxProperties {
    fn default() -> VkPhysicalDeviceSamplerFilterMinmaxProperties {
        VkPhysicalDeviceSamplerFilterMinmaxProperties {
            filter_minmax_single_component_formats: false,
            filter_minmax_image_component_mapping: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceSamplerFilterMinmaxProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceSamplerFilterMinmaxProperties {
    fn vk_free(&mut self) {
        
    }
}