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
use vulkan::amd::{VkShaderCorePropertiesFlags,RawVkShaderCorePropertiesFlags};

/// Wrapper for [VkPhysicalDeviceShaderCoreProperties2AMD](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShaderCoreProperties2 {
    pub shader_core_features: VkShaderCorePropertiesFlags,
    pub active_compute_unit_count: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShaderCoreProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub shader_core_features: RawVkShaderCorePropertiesFlags,
    pub active_compute_unit_count: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShaderCoreProperties2> for VkPhysicalDeviceShaderCoreProperties2 {
    fn vk_to_raw(src: &VkPhysicalDeviceShaderCoreProperties2, dst: &mut RawVkPhysicalDeviceShaderCoreProperties2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShaderCoreProperties2Amd);
        dst.next = ptr::null_mut();
        dst.shader_core_features = vk_to_raw_value(&src.shader_core_features);
        dst.active_compute_unit_count = vk_to_raw_value(&src.active_compute_unit_count);
    }
}

impl VkRawType<VkPhysicalDeviceShaderCoreProperties2> for RawVkPhysicalDeviceShaderCoreProperties2 {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShaderCoreProperties2) -> VkPhysicalDeviceShaderCoreProperties2 {
        VkPhysicalDeviceShaderCoreProperties2 {
            shader_core_features: RawVkShaderCorePropertiesFlags::vk_to_wrapped(&src.shader_core_features),
            active_compute_unit_count: u32::vk_to_wrapped(&src.active_compute_unit_count),
        }
    }
}

impl Default for VkPhysicalDeviceShaderCoreProperties2 {
    fn default() -> VkPhysicalDeviceShaderCoreProperties2 {
        VkPhysicalDeviceShaderCoreProperties2 {
            shader_core_features: Default::default(),
            active_compute_unit_count: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShaderCoreProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShaderCoreProperties2 {
    fn vk_free(&self) {
        
    }
}