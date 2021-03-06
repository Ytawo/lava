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
use vulkan::nv::{VkShadingRatePaletteEntry,RawVkShadingRatePaletteEntry};

/// Wrapper for [VkShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteNV.html).
#[derive(Debug, Clone)]
pub struct VkShadingRatePalette {
    pub shading_rate_palette_entries: Vec<VkShadingRatePaletteEntry>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkShadingRatePalette {
    pub shading_rate_palette_entry_count: u32,
    pub shading_rate_palette_entries: *mut RawVkShadingRatePaletteEntry,
}

impl VkWrappedType<RawVkShadingRatePalette> for VkShadingRatePalette {
    fn vk_to_raw(src: &VkShadingRatePalette, dst: &mut RawVkShadingRatePalette) {
        dst.shading_rate_palette_entry_count = src.shading_rate_palette_entries.len() as u32;
        dst.shading_rate_palette_entries = new_ptr_vk_array(&src.shading_rate_palette_entries);
    }
}

impl VkRawType<VkShadingRatePalette> for RawVkShadingRatePalette {
    fn vk_to_wrapped(src: &RawVkShadingRatePalette) -> VkShadingRatePalette {
        VkShadingRatePalette {
            shading_rate_palette_entries: new_vk_array(src.shading_rate_palette_entry_count, src.shading_rate_palette_entries),
        }
    }
}

impl Default for VkShadingRatePalette {
    fn default() -> VkShadingRatePalette {
        VkShadingRatePalette {
            shading_rate_palette_entries: Vec::new(),
        }
    }
}

impl VkSetup for VkShadingRatePalette {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkShadingRatePalette {
    fn vk_free(&self) {
        free_ptr(self.shading_rate_palette_entries);
    }
}