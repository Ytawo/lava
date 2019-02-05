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
use vulkan::vk::{VkImageFormatProperties,RawVkImageFormatProperties};
use vulkan::nv::{VkExternalMemoryFeatureFlags,RawVkExternalMemoryFeatureFlags};
use vulkan::nv::{VkExternalMemoryHandleTypeFlags,RawVkExternalMemoryHandleTypeFlags};

/// Wrapper for [VkExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalImageFormatPropertiesNV.html).
#[derive(Debug, Clone)]
pub struct VkExternalImageFormatProperties {
    pub image_format_properties: VkImageFormatProperties,
    pub external_memory_features: VkExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: VkExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: VkExternalMemoryHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExternalImageFormatProperties {
    pub image_format_properties: RawVkImageFormatProperties,
    pub external_memory_features: RawVkExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: RawVkExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: RawVkExternalMemoryHandleTypeFlags,
}

impl VkWrappedType<RawVkExternalImageFormatProperties> for VkExternalImageFormatProperties {
    fn vk_to_raw(src: &VkExternalImageFormatProperties, dst: &mut RawVkExternalImageFormatProperties) {
        dst.image_format_properties = vk_to_raw_value(&src.image_format_properties);
        dst.external_memory_features = vk_to_raw_value(&src.external_memory_features);
        dst.export_from_imported_handle_types = vk_to_raw_value(&src.export_from_imported_handle_types);
        dst.compatible_handle_types = vk_to_raw_value(&src.compatible_handle_types);
    }
}

impl VkRawType<VkExternalImageFormatProperties> for RawVkExternalImageFormatProperties {
    fn vk_to_wrapped(src: &RawVkExternalImageFormatProperties) -> VkExternalImageFormatProperties {
        VkExternalImageFormatProperties {
            image_format_properties: RawVkImageFormatProperties::vk_to_wrapped(&src.image_format_properties),
            external_memory_features: RawVkExternalMemoryFeatureFlags::vk_to_wrapped(&src.external_memory_features),
            export_from_imported_handle_types: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.export_from_imported_handle_types),
            compatible_handle_types: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.compatible_handle_types),
        }
    }
}

impl Default for VkExternalImageFormatProperties {
    fn default() -> VkExternalImageFormatProperties {
        VkExternalImageFormatProperties {
            image_format_properties: Default::default(),
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}

impl VkSetup for VkExternalImageFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.image_format_properties, fn_table);
    }
}

impl VkFree for RawVkExternalImageFormatProperties {
    fn vk_free(&self) {
        
    }
}