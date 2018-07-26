// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkCommandPoolTrimFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkCommandPoolTrimFlags {
    
}

impl VkRawType<VkCommandPoolTrimFlags> for RawVkCommandPoolTrimFlags {
    
    fn vk_to_wrapped(src: &RawVkCommandPoolTrimFlags) -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}

impl VkWrappedType<RawVkCommandPoolTrimFlags> for VkCommandPoolTrimFlags {
    
    fn vk_to_raw(src: &VkCommandPoolTrimFlags, dst: &mut RawVkCommandPoolTrimFlags) {
        *dst = 0;
    }
}

impl VkDefault for VkCommandPoolTrimFlags {
    
    fn vk_default() -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}