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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_blend_factor::*;
use vk::vk_blend_op::*;
use vk::vk_color_component_flags::*;

#[derive(Debug, Clone)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blend_enable: bool,
    pub src_color_blend_factor: VkBlendFactor,
    pub dst_color_blend_factor: VkBlendFactor,
    pub color_blend_op: VkBlendOp,
    pub src_alpha_blend_factor: VkBlendFactor,
    pub dst_alpha_blend_factor: VkBlendFactor,
    pub alpha_blend_op: VkBlendOp,
    pub color_write_mask: VkColorComponentFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineColorBlendAttachmentState {
    pub blend_enable: u32,
    pub src_color_blend_factor: RawVkBlendFactor,
    pub dst_color_blend_factor: RawVkBlendFactor,
    pub color_blend_op: RawVkBlendOp,
    pub src_alpha_blend_factor: RawVkBlendFactor,
    pub dst_alpha_blend_factor: RawVkBlendFactor,
    pub alpha_blend_op: RawVkBlendOp,
    pub color_write_mask: RawVkColorComponentFlags,
}

impl VkWrappedType<RawVkPipelineColorBlendAttachmentState> for VkPipelineColorBlendAttachmentState {
    fn vk_to_raw(src: &VkPipelineColorBlendAttachmentState, dst: &mut RawVkPipelineColorBlendAttachmentState) {
        dst.blend_enable = vk_to_raw_value(&src.blend_enable);
        dst.src_color_blend_factor = vk_to_raw_value(&src.src_color_blend_factor);
        dst.dst_color_blend_factor = vk_to_raw_value(&src.dst_color_blend_factor);
        dst.color_blend_op = vk_to_raw_value(&src.color_blend_op);
        dst.src_alpha_blend_factor = vk_to_raw_value(&src.src_alpha_blend_factor);
        dst.dst_alpha_blend_factor = vk_to_raw_value(&src.dst_alpha_blend_factor);
        dst.alpha_blend_op = vk_to_raw_value(&src.alpha_blend_op);
        dst.color_write_mask = vk_to_raw_value(&src.color_write_mask);
    }
}

impl VkRawType<VkPipelineColorBlendAttachmentState> for RawVkPipelineColorBlendAttachmentState {
    fn vk_to_wrapped(src: &RawVkPipelineColorBlendAttachmentState) -> VkPipelineColorBlendAttachmentState {
        VkPipelineColorBlendAttachmentState {
            blend_enable: u32::vk_to_wrapped(&src.blend_enable),
            src_color_blend_factor: RawVkBlendFactor::vk_to_wrapped(&src.src_color_blend_factor),
            dst_color_blend_factor: RawVkBlendFactor::vk_to_wrapped(&src.dst_color_blend_factor),
            color_blend_op: RawVkBlendOp::vk_to_wrapped(&src.color_blend_op),
            src_alpha_blend_factor: RawVkBlendFactor::vk_to_wrapped(&src.src_alpha_blend_factor),
            dst_alpha_blend_factor: RawVkBlendFactor::vk_to_wrapped(&src.dst_alpha_blend_factor),
            alpha_blend_op: RawVkBlendOp::vk_to_wrapped(&src.alpha_blend_op),
            color_write_mask: RawVkColorComponentFlags::vk_to_wrapped(&src.color_write_mask),
        }
    }
}

impl Default for VkPipelineColorBlendAttachmentState {
    fn default() -> VkPipelineColorBlendAttachmentState {
        VkPipelineColorBlendAttachmentState {
            blend_enable: false,
            src_color_blend_factor: VkBlendFactor::default(),
            dst_color_blend_factor: VkBlendFactor::default(),
            color_blend_op: VkBlendOp::default(),
            src_alpha_blend_factor: VkBlendFactor::default(),
            dst_alpha_blend_factor: VkBlendFactor::default(),
            alpha_blend_op: VkBlendOp::default(),
            color_write_mask: VkColorComponentFlags::default(),
        }
    }
}

impl VkSetup for VkPipelineColorBlendAttachmentState {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineColorBlendAttachmentState {
    fn vk_free(&mut self) {
        
    }
}