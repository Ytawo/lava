// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineViewportStateCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineViewportStateCreateFlags {
    
}

impl VkRawType<VkPipelineViewportStateCreateFlags> for RawVkPipelineViewportStateCreateFlags {
    
    fn vk_to_wrapped(src: &RawVkPipelineViewportStateCreateFlags) -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineViewportStateCreateFlags> for VkPipelineViewportStateCreateFlags {
    
    fn vk_to_raw(src: &VkPipelineViewportStateCreateFlags, dst: &mut RawVkPipelineViewportStateCreateFlags) {
        *dst = 0;
    }
}

impl VkDefault for VkPipelineViewportStateCreateFlags {
    
    fn vk_default() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}