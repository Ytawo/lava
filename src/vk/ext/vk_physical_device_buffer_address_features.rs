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
pub struct VkPhysicalDeviceBufferAddressFeatures {
    pub buffer_device_address: bool,
    pub buffer_device_address_capture_replay: bool,
    pub buffer_device_address_multi_device: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceBufferAddressFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub buffer_device_address: u32,
    pub buffer_device_address_capture_replay: u32,
    pub buffer_device_address_multi_device: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceBufferAddressFeatures> for VkPhysicalDeviceBufferAddressFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceBufferAddressFeatures, dst: &mut RawVkPhysicalDeviceBufferAddressFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceBufferAddressFeaturesExt);
        dst.next = ptr::null();
        dst.buffer_device_address = vk_to_raw_value(&src.buffer_device_address);
        dst.buffer_device_address_capture_replay = vk_to_raw_value(&src.buffer_device_address_capture_replay);
        dst.buffer_device_address_multi_device = vk_to_raw_value(&src.buffer_device_address_multi_device);
    }
}

impl VkRawType<VkPhysicalDeviceBufferAddressFeatures> for RawVkPhysicalDeviceBufferAddressFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceBufferAddressFeatures) -> VkPhysicalDeviceBufferAddressFeatures {
        VkPhysicalDeviceBufferAddressFeatures {
            buffer_device_address: u32::vk_to_wrapped(&src.buffer_device_address),
            buffer_device_address_capture_replay: u32::vk_to_wrapped(&src.buffer_device_address_capture_replay),
            buffer_device_address_multi_device: u32::vk_to_wrapped(&src.buffer_device_address_multi_device),
        }
    }
}

impl Default for VkPhysicalDeviceBufferAddressFeatures {
    fn default() -> VkPhysicalDeviceBufferAddressFeatures {
        VkPhysicalDeviceBufferAddressFeatures {
            buffer_device_address: false,
            buffer_device_address_capture_replay: false,
            buffer_device_address_multi_device: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceBufferAddressFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceBufferAddressFeatures {
    fn vk_free(&mut self) {
        
    }
}