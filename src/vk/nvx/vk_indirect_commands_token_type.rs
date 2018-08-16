// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkIndirectCommandsTokenType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkIndirectCommandsTokenType {
    Pipeline = 0,
    DescriptorSet = 1,
    IndexBuffer = 2,
    VertexBuffer = 3,
    PushConstant = 4,
    DrawIndexed = 5,
    Draw = 6,
    Dispatch = 7,
}

impl VkRawType<VkIndirectCommandsTokenType> for RawVkIndirectCommandsTokenType {
    fn vk_to_wrapped(src: &RawVkIndirectCommandsTokenType) -> VkIndirectCommandsTokenType {
        unsafe {
            *((src as *const i32) as *const VkIndirectCommandsTokenType)
        }
    }
}

impl VkWrappedType<RawVkIndirectCommandsTokenType> for VkIndirectCommandsTokenType {
    fn vk_to_raw(src: &VkIndirectCommandsTokenType, dst: &mut RawVkIndirectCommandsTokenType) {
        *dst = *src as i32
    }
}

impl Default for VkIndirectCommandsTokenType {
    fn default() -> VkIndirectCommandsTokenType {
        VkIndirectCommandsTokenType::Pipeline
    }
}