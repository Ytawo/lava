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

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMaintenance3Properties {
    pub max_per_set_descriptors: usize,
    pub max_memory_allocation_size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMaintenance3Properties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: u64,
}

impl VkWrappedType<RawVkPhysicalDeviceMaintenance3Properties> for VkPhysicalDeviceMaintenance3Properties {
    fn vk_to_raw(src: &VkPhysicalDeviceMaintenance3Properties, dst: &mut RawVkPhysicalDeviceMaintenance3Properties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMaintenance3Properties);
        dst.next = ptr::null();
        dst.max_per_set_descriptors = vk_to_raw_value(&src.max_per_set_descriptors);
        dst.max_memory_allocation_size = vk_to_raw_value(&src.max_memory_allocation_size);
    }
}

impl VkRawType<VkPhysicalDeviceMaintenance3Properties> for RawVkPhysicalDeviceMaintenance3Properties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMaintenance3Properties) -> VkPhysicalDeviceMaintenance3Properties {
        VkPhysicalDeviceMaintenance3Properties {
            max_per_set_descriptors: u32::vk_to_wrapped(&src.max_per_set_descriptors),
            max_memory_allocation_size: u64::vk_to_wrapped(&src.max_memory_allocation_size),
        }
    }
}

impl Default for VkPhysicalDeviceMaintenance3Properties {
    fn default() -> VkPhysicalDeviceMaintenance3Properties {
        VkPhysicalDeviceMaintenance3Properties {
            max_per_set_descriptors: 0,
            max_memory_allocation_size: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceMaintenance3Properties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMaintenance3Properties {
    fn vk_free(&mut self) {
        
    }
}