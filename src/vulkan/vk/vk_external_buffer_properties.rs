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
use vulkan::vk::{VkExternalMemoryProperties,RawVkExternalMemoryProperties};

/// Wrapper for [VkExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalBufferProperties.html).
#[derive(Debug, Clone)]
pub struct VkExternalBufferProperties {
    pub external_memory_properties: VkExternalMemoryProperties,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExternalBufferProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub external_memory_properties: RawVkExternalMemoryProperties,
}

impl VkWrappedType<RawVkExternalBufferProperties> for VkExternalBufferProperties {
    fn vk_to_raw(src: &VkExternalBufferProperties, dst: &mut RawVkExternalBufferProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExternalBufferProperties);
        dst.next = ptr::null();
        dst.external_memory_properties = vk_to_raw_value(&src.external_memory_properties);
    }
}

impl VkRawType<VkExternalBufferProperties> for RawVkExternalBufferProperties {
    fn vk_to_wrapped(src: &RawVkExternalBufferProperties) -> VkExternalBufferProperties {
        VkExternalBufferProperties {
            external_memory_properties: RawVkExternalMemoryProperties::vk_to_wrapped(&src.external_memory_properties),
        }
    }
}

impl Default for VkExternalBufferProperties {
    fn default() -> VkExternalBufferProperties {
        VkExternalBufferProperties {
            external_memory_properties: Default::default(),
        }
    }
}

impl VkSetup for VkExternalBufferProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.external_memory_properties, fn_table);
    }
}

impl VkFree for RawVkExternalBufferProperties {
    fn vk_free(&self) {
        
    }
}