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
use vk::vk_pipeline_color_blend_state_create_flags::*;
use vk::vk_logic_op::*;
use vk::vk_pipeline_color_blend_attachment_state::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineColorBlendStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineColorBlendStateCreateFlags,
    pub logic_op_enable: u32,
    pub logic_op: RawVkLogicOp,
    pub attachment_count: u32,
    pub attachments: *mut RawVkPipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo<'a> {
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logic_op_enable: bool,
    pub logic_op: VkLogicOp,
    pub attachments: &'a [VkPipelineColorBlendAttachmentState],
    pub blend_constants: [f32; 4],
}

impl<'a> VkWrappedType<RawVkPipelineColorBlendStateCreateInfo> for VkPipelineColorBlendStateCreateInfo<'a> {
    fn vk_to_raw(src: &VkPipelineColorBlendStateCreateInfo, dst: &mut RawVkPipelineColorBlendStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineColorBlendStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.logic_op_enable = vk_to_raw_value(&src.logic_op_enable);
        dst.logic_op = vk_to_raw_value(&src.logic_op);
        dst.attachment_count = src.attachments.len() as u32;
        dst.attachments = new_ptr_vk_array(src.attachments);
        dst.blend_constants = unsafe { let mut dst_array : [f32; 4] = mem::uninitialized(); to_array(&src.blend_constants, &mut dst_array); dst_array };
    }
}

impl Default for VkPipelineColorBlendStateCreateInfo<'static> {
    fn default() -> VkPipelineColorBlendStateCreateInfo<'static> {
        VkPipelineColorBlendStateCreateInfo {
            flags: VkPipelineColorBlendStateCreateFlags::default(),
            logic_op_enable: false,
            logic_op: VkLogicOp::default(),
            attachments: &[],
            blend_constants: [0.0; 4],
        }
    }
}

impl<'a> VkSetup for VkPipelineColorBlendStateCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineColorBlendStateCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.attachment_count as usize, self.attachments);
    }
}