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
use vk::vk_external_fence_handle_type_flags::*;

#[repr(C)]
pub struct RawVkPhysicalDeviceExternalFenceInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    handle_type: RawVkExternalFenceHandleTypeFlags,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceExternalFenceInfo {
    pub handle_type: VkExternalFenceHandleTypeFlags,
}

impl VkRawType<VkPhysicalDeviceExternalFenceInfo> for RawVkPhysicalDeviceExternalFenceInfo {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceExternalFenceInfo) -> VkPhysicalDeviceExternalFenceInfo {
        VkPhysicalDeviceExternalFenceInfo {
            handle_type: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.handle_type),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceExternalFenceInfo> for VkPhysicalDeviceExternalFenceInfo {
    fn vk_to_raw(src: &VkPhysicalDeviceExternalFenceInfo, dst: &mut RawVkPhysicalDeviceExternalFenceInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceExternalFenceInfo);
        dst.next = ptr::null();
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl Default for VkPhysicalDeviceExternalFenceInfo {
    fn default() -> VkPhysicalDeviceExternalFenceInfo {
        VkPhysicalDeviceExternalFenceInfo {
            handle_type: VkExternalFenceHandleTypeFlags::default(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceExternalFenceInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceExternalFenceInfo {
    fn vk_free(&mut self) {
        
    }
}