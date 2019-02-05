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

/// Wrapper for [VkTextureLODGatherFormatPropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html).
#[derive(Debug, Clone)]
pub struct VkTextureLODGatherFormatProperties {
    pub supports_texture_gather_lodbias_amd: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkTextureLODGatherFormatProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub supports_texture_gather_lodbias_amd: u32,
}

impl VkWrappedType<RawVkTextureLODGatherFormatProperties> for VkTextureLODGatherFormatProperties {
    fn vk_to_raw(src: &VkTextureLODGatherFormatProperties, dst: &mut RawVkTextureLODGatherFormatProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::TextureLodGatherFormatPropertiesAmd);
        dst.next = ptr::null();
        dst.supports_texture_gather_lodbias_amd = vk_to_raw_value(&src.supports_texture_gather_lodbias_amd);
    }
}

impl VkRawType<VkTextureLODGatherFormatProperties> for RawVkTextureLODGatherFormatProperties {
    fn vk_to_wrapped(src: &RawVkTextureLODGatherFormatProperties) -> VkTextureLODGatherFormatProperties {
        VkTextureLODGatherFormatProperties {
            supports_texture_gather_lodbias_amd: u32::vk_to_wrapped(&src.supports_texture_gather_lodbias_amd),
        }
    }
}

impl Default for VkTextureLODGatherFormatProperties {
    fn default() -> VkTextureLODGatherFormatProperties {
        VkTextureLODGatherFormatProperties {
            supports_texture_gather_lodbias_amd: false,
        }
    }
}

impl VkSetup for VkTextureLODGatherFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkTextureLODGatherFormatProperties {
    fn vk_free(&self) {
        
    }
}