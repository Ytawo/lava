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
use vulkan::vk::{VkFormatFeatureFlags,RawVkFormatFeatureFlags};

/// Wrapper for [VkDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkDrmFormatModifierProperties {
    pub drm_format_modifier: usize,
    pub drm_format_modifier_plane_count: usize,
    pub drm_format_modifier_tiling_features: VkFormatFeatureFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDrmFormatModifierProperties {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: RawVkFormatFeatureFlags,
}

impl VkWrappedType<RawVkDrmFormatModifierProperties> for VkDrmFormatModifierProperties {
    fn vk_to_raw(src: &VkDrmFormatModifierProperties, dst: &mut RawVkDrmFormatModifierProperties) {
        dst.drm_format_modifier = vk_to_raw_value(&src.drm_format_modifier);
        dst.drm_format_modifier_plane_count = vk_to_raw_value(&src.drm_format_modifier_plane_count);
        dst.drm_format_modifier_tiling_features = vk_to_raw_value(&src.drm_format_modifier_tiling_features);
    }
}

impl VkRawType<VkDrmFormatModifierProperties> for RawVkDrmFormatModifierProperties {
    fn vk_to_wrapped(src: &RawVkDrmFormatModifierProperties) -> VkDrmFormatModifierProperties {
        VkDrmFormatModifierProperties {
            drm_format_modifier: u64::vk_to_wrapped(&src.drm_format_modifier),
            drm_format_modifier_plane_count: u32::vk_to_wrapped(&src.drm_format_modifier_plane_count),
            drm_format_modifier_tiling_features: RawVkFormatFeatureFlags::vk_to_wrapped(&src.drm_format_modifier_tiling_features),
        }
    }
}

impl Default for VkDrmFormatModifierProperties {
    fn default() -> VkDrmFormatModifierProperties {
        VkDrmFormatModifierProperties {
            drm_format_modifier: 0,
            drm_format_modifier_plane_count: 0,
            drm_format_modifier_tiling_features: Default::default(),
        }
    }
}

impl VkSetup for VkDrmFormatModifierProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDrmFormatModifierProperties {
    fn vk_free(&self) {
        
    }
}