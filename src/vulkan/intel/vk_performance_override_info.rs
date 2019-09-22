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
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::intel::{VkPerformanceOverrideType,RawVkPerformanceOverrideType};

/// Wrapper for [VkPerformanceOverrideInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPerformanceOverrideInfoINTEL.html).
#[derive(Debug, Clone)]
pub struct VkPerformanceOverrideInfo {
    pub type_: VkPerformanceOverrideType,
    pub enable: bool,
    pub parameter: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPerformanceOverrideInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub type_: RawVkPerformanceOverrideType,
    pub enable: u32,
    pub parameter: u64,
}

impl VkWrappedType<RawVkPerformanceOverrideInfo> for VkPerformanceOverrideInfo {
    fn vk_to_raw(src: &VkPerformanceOverrideInfo, dst: &mut RawVkPerformanceOverrideInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PerformanceOverrideInfoIntel);
        dst.next = ptr::null_mut();
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.enable = vk_to_raw_value(&src.enable);
        dst.parameter = vk_to_raw_value(&src.parameter);
    }
}

impl VkRawType<VkPerformanceOverrideInfo> for RawVkPerformanceOverrideInfo {
    fn vk_to_wrapped(src: &RawVkPerformanceOverrideInfo) -> VkPerformanceOverrideInfo {
        VkPerformanceOverrideInfo {
            type_: RawVkPerformanceOverrideType::vk_to_wrapped(&src.type_),
            enable: u32::vk_to_wrapped(&src.enable),
            parameter: u64::vk_to_wrapped(&src.parameter),
        }
    }
}

impl Default for VkPerformanceOverrideInfo {
    fn default() -> VkPerformanceOverrideInfo {
        VkPerformanceOverrideInfo {
            type_: Default::default(),
            enable: false,
            parameter: 0,
        }
    }
}

impl VkSetup for VkPerformanceOverrideInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPerformanceOverrideInfo {
    fn vk_free(&self) {
        
    }
}