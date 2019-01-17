// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportSwizzleStateCreateFlags {
    
}

pub type RawVkPipelineViewportSwizzleStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineViewportSwizzleStateCreateFlags> for VkPipelineViewportSwizzleStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineViewportSwizzleStateCreateFlags, dst: &mut RawVkPipelineViewportSwizzleStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineViewportSwizzleStateCreateFlags> for RawVkPipelineViewportSwizzleStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineViewportSwizzleStateCreateFlags) -> VkPipelineViewportSwizzleStateCreateFlags {
        VkPipelineViewportSwizzleStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineViewportSwizzleStateCreateFlags {
    fn default() -> VkPipelineViewportSwizzleStateCreateFlags {
        VkPipelineViewportSwizzleStateCreateFlags {
            
        }
    }
}

impl VkPipelineViewportSwizzleStateCreateFlags {
    
    pub fn none() -> VkPipelineViewportSwizzleStateCreateFlags {
        VkPipelineViewportSwizzleStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineViewportSwizzleStateCreateFlags {
        VkPipelineViewportSwizzleStateCreateFlags {
            
        }
    }
}

impl VkPipelineViewportSwizzleStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}