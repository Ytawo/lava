// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineRasterizationStateCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineRasterizationStateCreateFlags {
    
}

impl VkRawType<VkPipelineRasterizationStateCreateFlags> for RawVkPipelineRasterizationStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationStateCreateFlags) -> VkPipelineRasterizationStateCreateFlags {
        VkPipelineRasterizationStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineRasterizationStateCreateFlags> for VkPipelineRasterizationStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineRasterizationStateCreateFlags, dst: &mut RawVkPipelineRasterizationStateCreateFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_PIPELINE_RASTERIZATION_STATE_CREATE_FLAGS : VkPipelineRasterizationStateCreateFlags = VkPipelineRasterizationStateCreateFlags {
    
};

impl VkDefault for VkPipelineRasterizationStateCreateFlags {
    fn vk_default() -> VkPipelineRasterizationStateCreateFlags {
        STATIC_VK_PIPELINE_RASTERIZATION_STATE_CREATE_FLAGS
    }
}