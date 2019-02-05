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

/// Wrapper for [VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeatures {
    pub compute_derivative_group_quads: bool,
    pub compute_derivative_group_linear: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceComputeShaderDerivativesFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub compute_derivative_group_quads: u32,
    pub compute_derivative_group_linear: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceComputeShaderDerivativesFeatures> for VkPhysicalDeviceComputeShaderDerivativesFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceComputeShaderDerivativesFeatures, dst: &mut RawVkPhysicalDeviceComputeShaderDerivativesFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceComputeShaderDerivativesFeaturesNv);
        dst.next = ptr::null();
        dst.compute_derivative_group_quads = vk_to_raw_value(&src.compute_derivative_group_quads);
        dst.compute_derivative_group_linear = vk_to_raw_value(&src.compute_derivative_group_linear);
    }
}

impl VkRawType<VkPhysicalDeviceComputeShaderDerivativesFeatures> for RawVkPhysicalDeviceComputeShaderDerivativesFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceComputeShaderDerivativesFeatures) -> VkPhysicalDeviceComputeShaderDerivativesFeatures {
        VkPhysicalDeviceComputeShaderDerivativesFeatures {
            compute_derivative_group_quads: u32::vk_to_wrapped(&src.compute_derivative_group_quads),
            compute_derivative_group_linear: u32::vk_to_wrapped(&src.compute_derivative_group_linear),
        }
    }
}

impl Default for VkPhysicalDeviceComputeShaderDerivativesFeatures {
    fn default() -> VkPhysicalDeviceComputeShaderDerivativesFeatures {
        VkPhysicalDeviceComputeShaderDerivativesFeatures {
            compute_derivative_group_quads: false,
            compute_derivative_group_linear: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceComputeShaderDerivativesFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceComputeShaderDerivativesFeatures {
    fn vk_free(&mut self) {
        
    }
}