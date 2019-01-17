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
use vk::vk_pipeline_dynamic_state_create_flags::*;
use vk::vk_dynamic_state::*;

#[derive(Debug, Clone)]
pub struct VkPipelineDynamicStateCreateInfo<'a> {
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamic_states: &'a [VkDynamicState],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineDynamicStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub dynamic_states: *mut RawVkDynamicState,
}

impl<'a> VkWrappedType<RawVkPipelineDynamicStateCreateInfo> for VkPipelineDynamicStateCreateInfo<'a> {
    fn vk_to_raw(src: &VkPipelineDynamicStateCreateInfo, dst: &mut RawVkPipelineDynamicStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineDynamicStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.dynamic_state_count = src.dynamic_states.len() as u32;
        dst.dynamic_states = new_ptr_vk_array(src.dynamic_states);
    }
}

impl Default for VkPipelineDynamicStateCreateInfo<'static> {
    fn default() -> VkPipelineDynamicStateCreateInfo<'static> {
        VkPipelineDynamicStateCreateInfo {
            flags: VkPipelineDynamicStateCreateFlags::default(),
            dynamic_states: &[],
        }
    }
}

impl<'a> VkSetup for VkPipelineDynamicStateCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineDynamicStateCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.dynamic_states);
    }
}