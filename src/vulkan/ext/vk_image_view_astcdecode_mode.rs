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
use vulkan::vk::{VkFormat,RawVkFormat};

/// Wrapper for [VkImageViewASTCDecodeModeEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageViewASTCDecodeModeEXT.html).
#[derive(Debug, Clone)]
pub struct VkImageViewASTCDecodeMode {
    pub decode_mode: VkFormat,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageViewASTCDecodeMode {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub decode_mode: RawVkFormat,
}

impl VkWrappedType<RawVkImageViewASTCDecodeMode> for VkImageViewASTCDecodeMode {
    fn vk_to_raw(src: &VkImageViewASTCDecodeMode, dst: &mut RawVkImageViewASTCDecodeMode) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageViewAstcDecodeModeExt);
        dst.next = ptr::null_mut();
        dst.decode_mode = vk_to_raw_value(&src.decode_mode);
    }
}

impl VkRawType<VkImageViewASTCDecodeMode> for RawVkImageViewASTCDecodeMode {
    fn vk_to_wrapped(src: &RawVkImageViewASTCDecodeMode) -> VkImageViewASTCDecodeMode {
        VkImageViewASTCDecodeMode {
            decode_mode: RawVkFormat::vk_to_wrapped(&src.decode_mode),
        }
    }
}

impl Default for VkImageViewASTCDecodeMode {
    fn default() -> VkImageViewASTCDecodeMode {
        VkImageViewASTCDecodeMode {
            decode_mode: Default::default(),
        }
    }
}

impl VkSetup for VkImageViewASTCDecodeMode {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkImageViewASTCDecodeMode {
    fn vk_free(&self) {
        
    }
}