// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineShaderStageCreateFlags {
    
}

pub type RawVkPipelineShaderStageCreateFlags = u32;

impl VkWrappedType<RawVkPipelineShaderStageCreateFlags> for VkPipelineShaderStageCreateFlags {
    fn vk_to_raw(src: &VkPipelineShaderStageCreateFlags, dst: &mut RawVkPipelineShaderStageCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineShaderStageCreateFlags> for RawVkPipelineShaderStageCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineShaderStageCreateFlags) -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

impl Default for VkPipelineShaderStageCreateFlags {
    fn default() -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

impl VkPipelineShaderStageCreateFlags {
    
    pub fn none() -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

impl VkPipelineShaderStageCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}