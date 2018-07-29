// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCreateFlags {
    disable_optimization: bool,
    allow_derivatives: bool,
    derivative: bool,
    view_index_from_device_index: bool,
    dispatch_base: bool,
}

impl VkRawType<VkPipelineCreateFlags> for RawVkPipelineCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCreateFlags) -> VkPipelineCreateFlags {
        VkPipelineCreateFlags {
            disable_optimization: (src & 0x00000001) != 0,
            allow_derivatives: (src & 0x00000002) != 0,
            derivative: (src & 0x00000004) != 0,
            view_index_from_device_index: (src & 0x00000008) != 0,
            dispatch_base: (src & 0x00000010) != 0,
        }
    }
}

impl VkWrappedType<RawVkPipelineCreateFlags> for VkPipelineCreateFlags {
    fn vk_to_raw(src: &VkPipelineCreateFlags, dst: &mut RawVkPipelineCreateFlags) {
        *dst = 0;
        if src.disable_optimization { *dst |= 0x00000001; }
        if src.allow_derivatives { *dst |= 0x00000002; }
        if src.derivative { *dst |= 0x00000004; }
        if src.view_index_from_device_index { *dst |= 0x00000008; }
        if src.dispatch_base { *dst |= 0x00000010; }
    }
}

pub static STATIC_VK_PIPELINE_CREATE_FLAGS : VkPipelineCreateFlags = VkPipelineCreateFlags {
    disable_optimization: false,
    allow_derivatives: false,
    derivative: false,
    view_index_from_device_index: false,
    dispatch_base: false,
};

impl VkDefault for VkPipelineCreateFlags {
    fn vk_default() -> VkPipelineCreateFlags {
        STATIC_VK_PIPELINE_CREATE_FLAGS
    }
}