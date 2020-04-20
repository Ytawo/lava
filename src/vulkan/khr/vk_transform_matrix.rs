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

/// Wrapper for [VkTransformMatrixKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html).
#[derive(Debug, Clone)]
pub struct VkTransformMatrix {
    pub matrix: [f32; 12],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkTransformMatrix {
    pub matrix: [f32; 12],
}

impl VkWrappedType<RawVkTransformMatrix> for VkTransformMatrix {
    fn vk_to_raw(src: &VkTransformMatrix, dst: &mut RawVkTransformMatrix) {
        dst.matrix = unsafe { let mut dst_array : [f32; 12] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.matrix, &mut dst_array); dst_array };
    }
}

impl VkRawType<VkTransformMatrix> for RawVkTransformMatrix {
    fn vk_to_wrapped(src: &RawVkTransformMatrix) -> VkTransformMatrix {
        VkTransformMatrix {
            matrix: unsafe { let mut dst_array : [f32; 12] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.matrix, &mut dst_array); dst_array },
        }
    }
}

impl Default for VkTransformMatrix {
    fn default() -> VkTransformMatrix {
        VkTransformMatrix {
            matrix: [0.0; 12],
        }
    }
}

impl VkSetup for VkTransformMatrix {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkTransformMatrix {
    fn vk_free(&self) {
        
    }
}