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
use vk::vk_version::*;
use vk::vk_physical_device_type::*;
use vk::vk_physical_device_limits::*;
use vk::vk_physical_device_sparse_properties::*;

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProperties {
    pub api_version: VkVersion,
    pub driver_version: u32,
    pub vendor_id: usize,
    pub device_id: usize,
    pub device_type: VkPhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct RawVkPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: RawVkPhysicalDeviceType,
    pub device_name: [c_char; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: RawVkPhysicalDeviceLimits,
    pub sparse_properties: RawVkPhysicalDeviceSparseProperties,
}

impl VkRawType<VkPhysicalDeviceProperties> for RawVkPhysicalDeviceProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceProperties) -> VkPhysicalDeviceProperties {
        VkPhysicalDeviceProperties {
            api_version: u32::vk_to_wrapped(&src.api_version),
            driver_version: src.driver_version,
            vendor_id: u32::vk_to_wrapped(&src.vendor_id),
            device_id: u32::vk_to_wrapped(&src.device_id),
            device_type: RawVkPhysicalDeviceType::vk_to_wrapped(&src.device_type),
            device_name: new_string(&src.device_name[0] as *const c_char),
            pipeline_cache_uuid: unsafe { let mut dst_array : [u8; 16] = mem::uninitialized(); to_array(&src.pipeline_cache_uuid, &mut dst_array); dst_array },
            limits: RawVkPhysicalDeviceLimits::vk_to_wrapped(&src.limits),
            sparse_properties: RawVkPhysicalDeviceSparseProperties::vk_to_wrapped(&src.sparse_properties),
        }
    }
}

impl VkSetup for VkPhysicalDeviceProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.limits, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.sparse_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkPhysicalDeviceProperties {
    fn vk_free(&mut self) {
        RawVkPhysicalDeviceLimits::vk_free(&mut self.limits);
        RawVkPhysicalDeviceSparseProperties::vk_free(&mut self.sparse_properties);
    }
}