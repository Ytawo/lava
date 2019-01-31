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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_extent_2d::*;

#[derive(Debug, Clone)]
pub struct VkDisplayModeParameters {
    pub visible_region: VkExtent2D,
    pub refresh_rate: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayModeParameters {
    pub visible_region: RawVkExtent2D,
    pub refresh_rate: u32,
}

impl VkWrappedType<RawVkDisplayModeParameters> for VkDisplayModeParameters {
    fn vk_to_raw(src: &VkDisplayModeParameters, dst: &mut RawVkDisplayModeParameters) {
        dst.visible_region = vk_to_raw_value(&src.visible_region);
        dst.refresh_rate = vk_to_raw_value(&src.refresh_rate);
    }
}

impl VkRawType<VkDisplayModeParameters> for RawVkDisplayModeParameters {
    fn vk_to_wrapped(src: &RawVkDisplayModeParameters) -> VkDisplayModeParameters {
        VkDisplayModeParameters {
            visible_region: RawVkExtent2D::vk_to_wrapped(&src.visible_region),
            refresh_rate: u32::vk_to_wrapped(&src.refresh_rate),
        }
    }
}

impl Default for VkDisplayModeParameters {
    fn default() -> VkDisplayModeParameters {
        VkDisplayModeParameters {
            visible_region: VkExtent2D::default(),
            refresh_rate: 0,
        }
    }
}

impl VkSetup for VkDisplayModeParameters {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.visible_region, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayModeParameters {
    fn vk_free(&mut self) {
        RawVkExtent2D::vk_free(&mut self.visible_region);
    }
}