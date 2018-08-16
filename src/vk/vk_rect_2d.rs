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
use vk::vk_offset_2d::*;
use vk::vk_extent_2d::*;

#[repr(C)]
pub struct RawVkRect2D {
    offset: RawVkOffset2D,
    extent: RawVkExtent2D,
}

#[derive(Debug, Clone)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

impl VkRawType<VkRect2D> for RawVkRect2D {
    fn vk_to_wrapped(src: &RawVkRect2D) -> VkRect2D {
        VkRect2D {
            offset: RawVkOffset2D::vk_to_wrapped(&src.offset),
            extent: RawVkExtent2D::vk_to_wrapped(&src.extent),
        }
    }
}

impl VkWrappedType<RawVkRect2D> for VkRect2D {
    fn vk_to_raw(src: &VkRect2D, dst: &mut RawVkRect2D) {
        dst.offset = vk_to_raw_value(&src.offset);
        dst.extent = vk_to_raw_value(&src.extent);
    }
}

impl Default for VkRect2D {
    fn default() -> VkRect2D {
        VkRect2D {
            offset: VkOffset2D::default(),
            extent: VkExtent2D::default(),
        }
    }
}

impl VkSetup for VkRect2D {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.offset, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkRect2D {
    fn vk_free(&mut self) {
        RawVkOffset2D::vk_free(&mut self.offset);
        RawVkExtent2D::vk_free(&mut self.extent);
    }
}