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
use vulkan::vk::{VkSharingMode,RawVkSharingMode};

/// Wrapper for [VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfo {
    pub drm_format_modifier: usize,
    pub sharing_mode: VkSharingMode,
    pub queue_family_indices: Vec<usize>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceImageDrmFormatModifierInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: RawVkSharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
}

impl VkWrappedType<RawVkPhysicalDeviceImageDrmFormatModifierInfo> for VkPhysicalDeviceImageDrmFormatModifierInfo {
    fn vk_to_raw(src: &VkPhysicalDeviceImageDrmFormatModifierInfo, dst: &mut RawVkPhysicalDeviceImageDrmFormatModifierInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceImageDrmFormatModifierInfoExt);
        dst.next = ptr::null();
        dst.drm_format_modifier = vk_to_raw_value(&src.drm_format_modifier);
        dst.sharing_mode = vk_to_raw_value(&src.sharing_mode);
        dst.queue_family_index_count = src.queue_family_indices.len() as u32;
        dst.queue_family_indices = new_ptr_vk_array(&src.queue_family_indices);
    }
}

impl VkRawType<VkPhysicalDeviceImageDrmFormatModifierInfo> for RawVkPhysicalDeviceImageDrmFormatModifierInfo {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceImageDrmFormatModifierInfo) -> VkPhysicalDeviceImageDrmFormatModifierInfo {
        VkPhysicalDeviceImageDrmFormatModifierInfo {
            drm_format_modifier: u64::vk_to_wrapped(&src.drm_format_modifier),
            sharing_mode: RawVkSharingMode::vk_to_wrapped(&src.sharing_mode),
            queue_family_indices: new_vk_array(src.queue_family_index_count, src.queue_family_indices),
        }
    }
}

impl Default for VkPhysicalDeviceImageDrmFormatModifierInfo {
    fn default() -> VkPhysicalDeviceImageDrmFormatModifierInfo {
        VkPhysicalDeviceImageDrmFormatModifierInfo {
            drm_format_modifier: 0,
            sharing_mode: Default::default(),
            queue_family_indices: Vec::new(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceImageDrmFormatModifierInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceImageDrmFormatModifierInfo {
    fn vk_free(&self) {
        free_ptr(self.queue_family_indices);
    }
}