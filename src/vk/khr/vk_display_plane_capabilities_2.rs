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
use vk::khr::vk_display_plane_capabilities::*;

#[repr(C)]
pub struct RawVkDisplayPlaneCapabilities2 {
    s_type: RawVkStructureType,
    next: *const c_void,
    capabilities: RawVkDisplayPlaneCapabilities,
}

#[derive(Debug, Clone)]
pub struct VkDisplayPlaneCapabilities2 {
    pub capabilities: VkDisplayPlaneCapabilities,
}

impl VkRawType<VkDisplayPlaneCapabilities2> for RawVkDisplayPlaneCapabilities2 {
    fn vk_to_wrapped(src: &RawVkDisplayPlaneCapabilities2) -> VkDisplayPlaneCapabilities2 {
        VkDisplayPlaneCapabilities2 {
            capabilities: RawVkDisplayPlaneCapabilities::vk_to_wrapped(&src.capabilities),
        }
    }
}

impl VkWrappedType<RawVkDisplayPlaneCapabilities2> for VkDisplayPlaneCapabilities2 {
    fn vk_to_raw(src: &VkDisplayPlaneCapabilities2, dst: &mut RawVkDisplayPlaneCapabilities2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayPlaneCapabilities2Khr);
        dst.next = ptr::null();
        dst.capabilities = vk_to_raw_value(&src.capabilities);
    }
}

impl Default for VkDisplayPlaneCapabilities2 {
    fn default() -> VkDisplayPlaneCapabilities2 {
        VkDisplayPlaneCapabilities2 {
            capabilities: VkDisplayPlaneCapabilities::default(),
        }
    }
}

impl VkSetup for VkDisplayPlaneCapabilities2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.capabilities, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayPlaneCapabilities2 {
    fn vk_free(&mut self) {
        RawVkDisplayPlaneCapabilities::vk_free(&mut self.capabilities);
    }
}