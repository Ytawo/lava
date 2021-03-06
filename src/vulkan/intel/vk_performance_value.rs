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
use vulkan::vk::*;
use vulkan::intel::{VkPerformanceValueType,RawVkPerformanceValueType};
use vulkan::intel::{VkPerformanceValueData,RawVkPerformanceValueData};

/// Wrapper for [VkPerformanceValueINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueINTEL.html).
#[derive(Debug, Clone)]
pub struct VkPerformanceValue {
    pub type_: VkPerformanceValueType,
    pub data: VkPerformanceValueData,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPerformanceValue {
    pub type_: RawVkPerformanceValueType,
    pub data: RawVkPerformanceValueData,
}

impl VkWrappedType<RawVkPerformanceValue> for VkPerformanceValue {
    fn vk_to_raw(src: &VkPerformanceValue, dst: &mut RawVkPerformanceValue) {
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.data = vk_to_raw_value(&src.data);
    }
}

impl VkRawType<VkPerformanceValue> for RawVkPerformanceValue {
    fn vk_to_wrapped(src: &RawVkPerformanceValue) -> VkPerformanceValue {
        VkPerformanceValue {
            type_: RawVkPerformanceValueType::vk_to_wrapped(&src.type_),
            data: RawVkPerformanceValueData::vk_to_wrapped(&src.data),
        }
    }
}

impl Default for VkPerformanceValue {
    fn default() -> VkPerformanceValue {
        VkPerformanceValue {
            type_: Default::default(),
            data: Default::default(),
        }
    }
}

impl VkSetup for VkPerformanceValue {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPerformanceValue {
    fn vk_free(&self) {
        
    }
}