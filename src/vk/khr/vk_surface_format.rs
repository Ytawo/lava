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
use vk::vk_format::*;
use vk::khr::vk_color_space::*;

#[derive(Debug, Clone)]
pub struct VkSurfaceFormat {
    pub format: VkFormat,
    pub color_space: VkColorSpace,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSurfaceFormat {
    pub format: RawVkFormat,
    pub color_space: RawVkColorSpace,
}

impl VkWrappedType<RawVkSurfaceFormat> for VkSurfaceFormat {
    fn vk_to_raw(src: &VkSurfaceFormat, dst: &mut RawVkSurfaceFormat) {
        dst.format = vk_to_raw_value(&src.format);
        dst.color_space = vk_to_raw_value(&src.color_space);
    }
}

impl VkRawType<VkSurfaceFormat> for RawVkSurfaceFormat {
    fn vk_to_wrapped(src: &RawVkSurfaceFormat) -> VkSurfaceFormat {
        VkSurfaceFormat {
            format: RawVkFormat::vk_to_wrapped(&src.format),
            color_space: RawVkColorSpace::vk_to_wrapped(&src.color_space),
        }
    }
}

impl Default for VkSurfaceFormat {
    fn default() -> VkSurfaceFormat {
        VkSurfaceFormat {
            format: VkFormat::default(),
            color_space: VkColorSpace::default(),
        }
    }
}

impl VkSetup for VkSurfaceFormat {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSurfaceFormat {
    fn vk_free(&mut self) {
        
    }
}