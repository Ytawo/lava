// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineLayoutCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineLayoutCreateFlags {
    
}

impl VkRawType<VkPipelineLayoutCreateFlags> for RawVkPipelineLayoutCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineLayoutCreateFlags) -> VkPipelineLayoutCreateFlags {
        VkPipelineLayoutCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineLayoutCreateFlags> for VkPipelineLayoutCreateFlags {
    fn vk_to_raw(src: &VkPipelineLayoutCreateFlags, dst: &mut RawVkPipelineLayoutCreateFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_PIPELINE_LAYOUT_CREATE_FLAGS : VkPipelineLayoutCreateFlags = VkPipelineLayoutCreateFlags {
    
};

impl VkDefault for VkPipelineLayoutCreateFlags {
    fn vk_default() -> VkPipelineLayoutCreateFlags {
        STATIC_VK_PIPELINE_LAYOUT_CREATE_FLAGS
    }
}