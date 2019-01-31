// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineMultisampleStateCreateFlags {
    
}

pub type RawVkPipelineMultisampleStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineMultisampleStateCreateFlags> for VkPipelineMultisampleStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineMultisampleStateCreateFlags, dst: &mut RawVkPipelineMultisampleStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineMultisampleStateCreateFlags> for RawVkPipelineMultisampleStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineMultisampleStateCreateFlags) -> VkPipelineMultisampleStateCreateFlags {
        VkPipelineMultisampleStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineMultisampleStateCreateFlags {
    fn default() -> VkPipelineMultisampleStateCreateFlags {
        VkPipelineMultisampleStateCreateFlags {
            
        }
    }
}

impl VkPipelineMultisampleStateCreateFlags {
    
    pub fn none() -> VkPipelineMultisampleStateCreateFlags {
        VkPipelineMultisampleStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineMultisampleStateCreateFlags {
        VkPipelineMultisampleStateCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkPipelineMultisampleStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineMultisampleStateCreateFlags {
            $($x: true,)*
            ..VkPipelineMultisampleStateCreateFlags::none()
        }
    }
}

impl VkPipelineMultisampleStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}