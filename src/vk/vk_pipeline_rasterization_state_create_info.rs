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
use vk::vk_pipeline_rasterization_state_create_flags::*;
use vk::vk_polygon_mode::*;
use vk::vk_cull_mode_flags::*;
use vk::vk_front_face::*;

#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: bool,
    pub rasterizer_discard_enable: bool,
    pub polygon_mode: VkPolygonMode,
    pub cull_mode: VkCullModeFlags,
    pub front_face: VkFrontFace,
    pub depth_bias_enable: bool,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineRasterizationStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: u32,
    pub rasterizer_discard_enable: u32,
    pub polygon_mode: RawVkPolygonMode,
    pub cull_mode: RawVkCullModeFlags,
    pub front_face: RawVkFrontFace,
    pub depth_bias_enable: u32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

impl VkWrappedType<RawVkPipelineRasterizationStateCreateInfo> for VkPipelineRasterizationStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineRasterizationStateCreateInfo, dst: &mut RawVkPipelineRasterizationStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineRasterizationStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.depth_clamp_enable = vk_to_raw_value(&src.depth_clamp_enable);
        dst.rasterizer_discard_enable = vk_to_raw_value(&src.rasterizer_discard_enable);
        dst.polygon_mode = vk_to_raw_value(&src.polygon_mode);
        dst.cull_mode = vk_to_raw_value(&src.cull_mode);
        dst.front_face = vk_to_raw_value(&src.front_face);
        dst.depth_bias_enable = vk_to_raw_value(&src.depth_bias_enable);
        dst.depth_bias_constant_factor = src.depth_bias_constant_factor;
        dst.depth_bias_clamp = src.depth_bias_clamp;
        dst.depth_bias_slope_factor = src.depth_bias_slope_factor;
        dst.line_width = src.line_width;
    }
}

impl VkRawType<VkPipelineRasterizationStateCreateInfo> for RawVkPipelineRasterizationStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationStateCreateInfo) -> VkPipelineRasterizationStateCreateInfo {
        VkPipelineRasterizationStateCreateInfo {
            flags: RawVkPipelineRasterizationStateCreateFlags::vk_to_wrapped(&src.flags),
            depth_clamp_enable: u32::vk_to_wrapped(&src.depth_clamp_enable),
            rasterizer_discard_enable: u32::vk_to_wrapped(&src.rasterizer_discard_enable),
            polygon_mode: RawVkPolygonMode::vk_to_wrapped(&src.polygon_mode),
            cull_mode: RawVkCullModeFlags::vk_to_wrapped(&src.cull_mode),
            front_face: RawVkFrontFace::vk_to_wrapped(&src.front_face),
            depth_bias_enable: u32::vk_to_wrapped(&src.depth_bias_enable),
            depth_bias_constant_factor: src.depth_bias_constant_factor,
            depth_bias_clamp: src.depth_bias_clamp,
            depth_bias_slope_factor: src.depth_bias_slope_factor,
            line_width: src.line_width,
        }
    }
}

impl Default for VkPipelineRasterizationStateCreateInfo {
    fn default() -> VkPipelineRasterizationStateCreateInfo {
        VkPipelineRasterizationStateCreateInfo {
            flags: VkPipelineRasterizationStateCreateFlags::default(),
            depth_clamp_enable: false,
            rasterizer_discard_enable: false,
            polygon_mode: VkPolygonMode::default(),
            cull_mode: VkCullModeFlags::default(),
            front_face: VkFrontFace::default(),
            depth_bias_enable: false,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 0.0,
        }
    }
}

impl VkSetup for VkPipelineRasterizationStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineRasterizationStateCreateInfo {
    fn vk_free(&mut self) {
        
    }
}