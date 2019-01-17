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
use vk::khr::vk_display_properties::*;

#[derive(Debug, Clone)]
pub struct VkDisplayProperties2 {
    pub display_properties: VkDisplayProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub display_properties: RawVkDisplayProperties,
}

impl VkRawType<VkDisplayProperties2> for RawVkDisplayProperties2 {
    fn vk_to_wrapped(src: &RawVkDisplayProperties2) -> VkDisplayProperties2 {
        VkDisplayProperties2 {
            display_properties: RawVkDisplayProperties::vk_to_wrapped(&src.display_properties),
        }
    }
}

impl VkSetup for VkDisplayProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.display_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayProperties2 {
    fn vk_free(&mut self) {
        RawVkDisplayProperties::vk_free(&mut self.display_properties);
    }
}