// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineInputAssemblyStateCreateFlags {
    
}

pub type RawVkPipelineInputAssemblyStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineInputAssemblyStateCreateFlags> for VkPipelineInputAssemblyStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineInputAssemblyStateCreateFlags, dst: &mut RawVkPipelineInputAssemblyStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineInputAssemblyStateCreateFlags> for RawVkPipelineInputAssemblyStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineInputAssemblyStateCreateFlags) -> VkPipelineInputAssemblyStateCreateFlags {
        VkPipelineInputAssemblyStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineInputAssemblyStateCreateFlags {
    fn default() -> VkPipelineInputAssemblyStateCreateFlags {
        VkPipelineInputAssemblyStateCreateFlags {
            
        }
    }
}

impl VkPipelineInputAssemblyStateCreateFlags {
    
    pub fn none() -> VkPipelineInputAssemblyStateCreateFlags {
        VkPipelineInputAssemblyStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineInputAssemblyStateCreateFlags {
        VkPipelineInputAssemblyStateCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkPipelineInputAssemblyStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineInputAssemblyStateCreateFlags {
            $($x: true,)*
            ..VkPipelineInputAssemblyStateCreateFlags::none()
        }
    }
}

impl VkPipelineInputAssemblyStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}