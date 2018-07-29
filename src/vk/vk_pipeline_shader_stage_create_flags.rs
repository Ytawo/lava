// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineShaderStageCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineShaderStageCreateFlags {
    
}

impl VkRawType<VkPipelineShaderStageCreateFlags> for RawVkPipelineShaderStageCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineShaderStageCreateFlags) -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineShaderStageCreateFlags> for VkPipelineShaderStageCreateFlags {
    fn vk_to_raw(src: &VkPipelineShaderStageCreateFlags, dst: &mut RawVkPipelineShaderStageCreateFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_PIPELINE_SHADER_STAGE_CREATE_FLAGS : VkPipelineShaderStageCreateFlags = VkPipelineShaderStageCreateFlags {
    
};

impl VkDefault for VkPipelineShaderStageCreateFlags {
    fn vk_default() -> VkPipelineShaderStageCreateFlags {
        STATIC_VK_PIPELINE_SHADER_STAGE_CREATE_FLAGS
    }
}