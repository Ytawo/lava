// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolTrimFlags {
    
}

pub type RawVkCommandPoolTrimFlags = u32;

impl VkWrappedType<RawVkCommandPoolTrimFlags> for VkCommandPoolTrimFlags {
    fn vk_to_raw(src: &VkCommandPoolTrimFlags, dst: &mut RawVkCommandPoolTrimFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkCommandPoolTrimFlags> for RawVkCommandPoolTrimFlags {
    fn vk_to_wrapped(src: &RawVkCommandPoolTrimFlags) -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}

impl Default for VkCommandPoolTrimFlags {
    fn default() -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}

impl VkCommandPoolTrimFlags {
    
    pub fn none() -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
    
    pub fn all() -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}

impl VkCommandPoolTrimFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}