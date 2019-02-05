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
use vulkan::ext::{VkValidationCacheCreateFlags,RawVkValidationCacheCreateFlags};

/// Wrapper for [VkValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkValidationCacheCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkValidationCacheCreateInfo<'a> {
    pub flags: VkValidationCacheCreateFlags,
    pub initial_data: &'a [c_void],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkValidationCacheCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkValidationCacheCreateFlags,
    pub initial_data_size: usize,
    pub initial_data: *mut c_void,
}

impl<'a> VkWrappedType<RawVkValidationCacheCreateInfo> for VkValidationCacheCreateInfo<'a> {
    fn vk_to_raw(src: &VkValidationCacheCreateInfo, dst: &mut RawVkValidationCacheCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ValidationCacheCreateInfoExt);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.initial_data_size = src.initial_data.len();
        dst.initial_data = get_vec_ptr(src.initial_data);
    }
}

impl<'a> VkRawType<VkValidationCacheCreateInfo<'a>> for RawVkValidationCacheCreateInfo {
    fn vk_to_wrapped(src: &RawVkValidationCacheCreateInfo) -> VkValidationCacheCreateInfo<'a> {
        VkValidationCacheCreateInfo {
            flags: RawVkValidationCacheCreateFlags::vk_to_wrapped(&src.flags),
            initial_data: slice_from_ptr(src.initial_data_size as usize, src.initial_data),
        }
    }
}

impl Default for VkValidationCacheCreateInfo<'static> {
    fn default() -> VkValidationCacheCreateInfo<'static> {
        VkValidationCacheCreateInfo {
            flags: Default::default(),
            initial_data: &[],
        }
    }
}

impl<'a> VkSetup for VkValidationCacheCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkValidationCacheCreateInfo {
    fn vk_free(&self) {
        
    }
}