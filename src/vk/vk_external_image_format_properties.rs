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
use vk::vk_structure_type::*;
use vk::vk_external_memory_properties::*;

#[repr(C)]
pub struct RawVkExternalImageFormatProperties {
    s_type: RawVkStructureType,
    next: *const c_void,
    external_memory_properties: RawVkExternalMemoryProperties,
}

#[derive(Debug, Clone)]
pub struct VkExternalImageFormatProperties {
    pub external_memory_properties: VkExternalMemoryProperties,
}

impl VkRawType<VkExternalImageFormatProperties> for RawVkExternalImageFormatProperties {
    fn vk_to_wrapped(src: &RawVkExternalImageFormatProperties) -> VkExternalImageFormatProperties {
        VkExternalImageFormatProperties {
            external_memory_properties: RawVkExternalMemoryProperties::vk_to_wrapped(&src.external_memory_properties),
        }
    }
}

impl VkWrappedType<RawVkExternalImageFormatProperties> for VkExternalImageFormatProperties {
    fn vk_to_raw(src: &VkExternalImageFormatProperties, dst: &mut RawVkExternalImageFormatProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExternalImageFormatProperties);
        dst.next = ptr::null();
        dst.external_memory_properties = vk_to_raw_value(&src.external_memory_properties);
    }
}

impl Default for VkExternalImageFormatProperties {
    fn default() -> VkExternalImageFormatProperties {
        VkExternalImageFormatProperties {
            external_memory_properties: VkExternalMemoryProperties::default(),
        }
    }
}

impl VkSetup for VkExternalImageFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.external_memory_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkExternalImageFormatProperties {
    fn vk_free(&mut self) {
        RawVkExternalMemoryProperties::vk_free(&mut self.external_memory_properties);
    }
}