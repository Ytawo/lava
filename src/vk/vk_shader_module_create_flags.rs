// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkShaderModuleCreateFlags {
    
}

pub type RawVkShaderModuleCreateFlags = u32;

impl VkWrappedType<RawVkShaderModuleCreateFlags> for VkShaderModuleCreateFlags {
    fn vk_to_raw(src: &VkShaderModuleCreateFlags, dst: &mut RawVkShaderModuleCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkShaderModuleCreateFlags> for RawVkShaderModuleCreateFlags {
    fn vk_to_wrapped(src: &RawVkShaderModuleCreateFlags) -> VkShaderModuleCreateFlags {
        VkShaderModuleCreateFlags {
            
        }
    }
}

impl Default for VkShaderModuleCreateFlags {
    fn default() -> VkShaderModuleCreateFlags {
        VkShaderModuleCreateFlags {
            
        }
    }
}

impl VkShaderModuleCreateFlags {
    
    pub fn none() -> VkShaderModuleCreateFlags {
        VkShaderModuleCreateFlags {
            
        }
    }
    
    pub fn all() -> VkShaderModuleCreateFlags {
        VkShaderModuleCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkShaderModuleCreateFlags {
    ( $( $x:ident ),* ) => {
        VkShaderModuleCreateFlags {
            $($x: true,)*
            ..VkShaderModuleCreateFlags::none()
        }
    }
}

impl VkShaderModuleCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}