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
use vk::nvx::vk_indirect_commands_token_type::*;
use vk::vk_buffer::*;

#[derive(Debug, Clone)]
pub struct VkIndirectCommandsToken<'a> {
    pub token_type: VkIndirectCommandsTokenType,
    pub buffer: &'a VkBuffer,
    pub offset: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkIndirectCommandsToken {
    pub token_type: RawVkIndirectCommandsTokenType,
    pub buffer: RawVkBuffer,
    pub offset: u64,
}

impl<'a> VkWrappedType<RawVkIndirectCommandsToken> for VkIndirectCommandsToken<'a> {
    fn vk_to_raw(src: &VkIndirectCommandsToken, dst: &mut RawVkIndirectCommandsToken) {
        dst.token_type = vk_to_raw_value(&src.token_type);
        dst.buffer = vk_to_raw_value(src.buffer);
        dst.offset = vk_to_raw_value(&src.offset);
    }
}

impl Default for VkIndirectCommandsToken<'static> {
    fn default() -> VkIndirectCommandsToken<'static> {
        VkIndirectCommandsToken {
            token_type: VkIndirectCommandsTokenType::default(),
            buffer: vk_null_ref(),
            offset: 0,
        }
    }
}

impl<'a> VkSetup for VkIndirectCommandsToken<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkIndirectCommandsToken {
    fn vk_free(&mut self) {
        
    }
}