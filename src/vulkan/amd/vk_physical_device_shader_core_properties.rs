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

/// Wrapper for [VkPhysicalDeviceShaderCorePropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShaderCoreProperties {
    pub shader_engine_count: usize,
    pub shader_arrays_per_engine_count: usize,
    pub compute_units_per_shader_array: usize,
    pub simd_per_compute_unit: usize,
    pub wavefronts_per_simd: usize,
    pub wavefront_size: usize,
    pub sgprs_per_simd: usize,
    pub min_sgpr_allocation: usize,
    pub max_sgpr_allocation: usize,
    pub sgpr_allocation_granularity: usize,
    pub vgprs_per_simd: usize,
    pub min_vgpr_allocation: usize,
    pub max_vgpr_allocation: usize,
    pub vgpr_allocation_granularity: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShaderCoreProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShaderCoreProperties> for VkPhysicalDeviceShaderCoreProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceShaderCoreProperties, dst: &mut RawVkPhysicalDeviceShaderCoreProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShaderCorePropertiesAmd);
        dst.next = ptr::null();
        dst.shader_engine_count = vk_to_raw_value(&src.shader_engine_count);
        dst.shader_arrays_per_engine_count = vk_to_raw_value(&src.shader_arrays_per_engine_count);
        dst.compute_units_per_shader_array = vk_to_raw_value(&src.compute_units_per_shader_array);
        dst.simd_per_compute_unit = vk_to_raw_value(&src.simd_per_compute_unit);
        dst.wavefronts_per_simd = vk_to_raw_value(&src.wavefronts_per_simd);
        dst.wavefront_size = vk_to_raw_value(&src.wavefront_size);
        dst.sgprs_per_simd = vk_to_raw_value(&src.sgprs_per_simd);
        dst.min_sgpr_allocation = vk_to_raw_value(&src.min_sgpr_allocation);
        dst.max_sgpr_allocation = vk_to_raw_value(&src.max_sgpr_allocation);
        dst.sgpr_allocation_granularity = vk_to_raw_value(&src.sgpr_allocation_granularity);
        dst.vgprs_per_simd = vk_to_raw_value(&src.vgprs_per_simd);
        dst.min_vgpr_allocation = vk_to_raw_value(&src.min_vgpr_allocation);
        dst.max_vgpr_allocation = vk_to_raw_value(&src.max_vgpr_allocation);
        dst.vgpr_allocation_granularity = vk_to_raw_value(&src.vgpr_allocation_granularity);
    }
}

impl VkRawType<VkPhysicalDeviceShaderCoreProperties> for RawVkPhysicalDeviceShaderCoreProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShaderCoreProperties) -> VkPhysicalDeviceShaderCoreProperties {
        VkPhysicalDeviceShaderCoreProperties {
            shader_engine_count: u32::vk_to_wrapped(&src.shader_engine_count),
            shader_arrays_per_engine_count: u32::vk_to_wrapped(&src.shader_arrays_per_engine_count),
            compute_units_per_shader_array: u32::vk_to_wrapped(&src.compute_units_per_shader_array),
            simd_per_compute_unit: u32::vk_to_wrapped(&src.simd_per_compute_unit),
            wavefronts_per_simd: u32::vk_to_wrapped(&src.wavefronts_per_simd),
            wavefront_size: u32::vk_to_wrapped(&src.wavefront_size),
            sgprs_per_simd: u32::vk_to_wrapped(&src.sgprs_per_simd),
            min_sgpr_allocation: u32::vk_to_wrapped(&src.min_sgpr_allocation),
            max_sgpr_allocation: u32::vk_to_wrapped(&src.max_sgpr_allocation),
            sgpr_allocation_granularity: u32::vk_to_wrapped(&src.sgpr_allocation_granularity),
            vgprs_per_simd: u32::vk_to_wrapped(&src.vgprs_per_simd),
            min_vgpr_allocation: u32::vk_to_wrapped(&src.min_vgpr_allocation),
            max_vgpr_allocation: u32::vk_to_wrapped(&src.max_vgpr_allocation),
            vgpr_allocation_granularity: u32::vk_to_wrapped(&src.vgpr_allocation_granularity),
        }
    }
}

impl Default for VkPhysicalDeviceShaderCoreProperties {
    fn default() -> VkPhysicalDeviceShaderCoreProperties {
        VkPhysicalDeviceShaderCoreProperties {
            shader_engine_count: 0,
            shader_arrays_per_engine_count: 0,
            compute_units_per_shader_array: 0,
            simd_per_compute_unit: 0,
            wavefronts_per_simd: 0,
            wavefront_size: 0,
            sgprs_per_simd: 0,
            min_sgpr_allocation: 0,
            max_sgpr_allocation: 0,
            sgpr_allocation_granularity: 0,
            vgprs_per_simd: 0,
            min_vgpr_allocation: 0,
            max_vgpr_allocation: 0,
            vgpr_allocation_granularity: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShaderCoreProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShaderCoreProperties {
    fn vk_free(&mut self) {
        
    }
}