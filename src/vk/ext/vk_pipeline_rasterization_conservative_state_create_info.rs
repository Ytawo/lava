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
use vk::ext::vk_pipeline_rasterization_conservative_state_create_flags::*;
use vk::ext::vk_conservative_rasterization_mode::*;

#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationConservativeStateCreateInfo {
    pub flags: VkPipelineRasterizationConservativeStateCreateFlags,
    pub conservative_rasterization_mode: VkConservativeRasterizationMode,
    pub extra_primitive_overestimation_size: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineRasterizationConservativeStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineRasterizationConservativeStateCreateFlags,
    pub conservative_rasterization_mode: RawVkConservativeRasterizationMode,
    pub extra_primitive_overestimation_size: f32,
}

impl VkWrappedType<RawVkPipelineRasterizationConservativeStateCreateInfo> for VkPipelineRasterizationConservativeStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineRasterizationConservativeStateCreateInfo, dst: &mut RawVkPipelineRasterizationConservativeStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineRasterizationConservativeStateCreateInfoExt);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.conservative_rasterization_mode = vk_to_raw_value(&src.conservative_rasterization_mode);
        dst.extra_primitive_overestimation_size = src.extra_primitive_overestimation_size;
    }
}

impl VkRawType<VkPipelineRasterizationConservativeStateCreateInfo> for RawVkPipelineRasterizationConservativeStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationConservativeStateCreateInfo) -> VkPipelineRasterizationConservativeStateCreateInfo {
        VkPipelineRasterizationConservativeStateCreateInfo {
            flags: RawVkPipelineRasterizationConservativeStateCreateFlags::vk_to_wrapped(&src.flags),
            conservative_rasterization_mode: RawVkConservativeRasterizationMode::vk_to_wrapped(&src.conservative_rasterization_mode),
            extra_primitive_overestimation_size: src.extra_primitive_overestimation_size,
        }
    }
}

impl Default for VkPipelineRasterizationConservativeStateCreateInfo {
    fn default() -> VkPipelineRasterizationConservativeStateCreateInfo {
        VkPipelineRasterizationConservativeStateCreateInfo {
            flags: VkPipelineRasterizationConservativeStateCreateFlags::default(),
            conservative_rasterization_mode: VkConservativeRasterizationMode::default(),
            extra_primitive_overestimation_size: 0.0,
        }
    }
}

impl VkSetup for VkPipelineRasterizationConservativeStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineRasterizationConservativeStateCreateInfo {
    fn vk_free(&mut self) {
        
    }
}