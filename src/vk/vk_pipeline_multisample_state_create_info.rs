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
use vk::vk_pipeline_multisample_state_create_flags::*;
use vk::vk_sample_count_flags::*;

#[repr(C)]
pub struct RawVkPipelineMultisampleStateCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkPipelineMultisampleStateCreateFlags,
    rasterization_samples: RawVkSampleCountFlags,
    sample_shading_enable: u32,
    min_sample_shading: f32,
    sample_mask: *const u32,
    alpha_to_coverage_enable: u32,
    alpha_to_one_enable: u32,
}

#[derive(Debug, Clone)]
pub struct VkPipelineMultisampleStateCreateInfo<'a> {
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterization_samples: VkSampleCountFlags,
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,
    pub sample_mask: Option<&'a [u32]>,
    pub alpha_to_coverage_enable: bool,
    pub alpha_to_one_enable: bool,
}

impl<'a> VkWrappedType<RawVkPipelineMultisampleStateCreateInfo> for VkPipelineMultisampleStateCreateInfo<'a> {
    fn vk_to_raw(src: &VkPipelineMultisampleStateCreateInfo, dst: &mut RawVkPipelineMultisampleStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineMultisampleStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.rasterization_samples = vk_to_raw_value(&src.rasterization_samples);
        dst.sample_shading_enable = vk_to_raw_value(&src.sample_shading_enable);
        dst.min_sample_shading = src.min_sample_shading;
        dst.sample_mask = slice_option_to_ptr(src.sample_mask);
        dst.alpha_to_coverage_enable = vk_to_raw_value(&src.alpha_to_coverage_enable);
        dst.alpha_to_one_enable = vk_to_raw_value(&src.alpha_to_one_enable);
    }
}

impl Default for VkPipelineMultisampleStateCreateInfo<'static> {
    fn default() -> VkPipelineMultisampleStateCreateInfo<'static> {
        VkPipelineMultisampleStateCreateInfo {
            flags: VkPipelineMultisampleStateCreateFlags::default(),
            rasterization_samples: VkSampleCountFlags::default(),
            sample_shading_enable: false,
            min_sample_shading: 0.0,
            sample_mask: None,
            alpha_to_coverage_enable: false,
            alpha_to_one_enable: false,
        }
    }
}

impl<'a> VkSetup for VkPipelineMultisampleStateCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineMultisampleStateCreateInfo {
    fn vk_free(&mut self) {
        
    }
}