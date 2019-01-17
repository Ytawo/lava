// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineLayoutCreateFlags {
    
}

pub type RawVkPipelineLayoutCreateFlags = u32;

impl VkWrappedType<RawVkPipelineLayoutCreateFlags> for VkPipelineLayoutCreateFlags {
    fn vk_to_raw(src: &VkPipelineLayoutCreateFlags, dst: &mut RawVkPipelineLayoutCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineLayoutCreateFlags> for RawVkPipelineLayoutCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineLayoutCreateFlags) -> VkPipelineLayoutCreateFlags {
        VkPipelineLayoutCreateFlags {
            
        }
    }
}

impl Default for VkPipelineLayoutCreateFlags {
    fn default() -> VkPipelineLayoutCreateFlags {
        VkPipelineLayoutCreateFlags {
            
        }
    }
}

impl VkPipelineLayoutCreateFlags {
    
    pub fn none() -> VkPipelineLayoutCreateFlags {
        VkPipelineLayoutCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineLayoutCreateFlags {
        VkPipelineLayoutCreateFlags {
            
        }
    }
}

impl VkPipelineLayoutCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}