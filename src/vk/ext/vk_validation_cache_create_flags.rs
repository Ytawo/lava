// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkValidationCacheCreateFlags {
    
}

pub type RawVkValidationCacheCreateFlags = u32;

impl VkWrappedType<RawVkValidationCacheCreateFlags> for VkValidationCacheCreateFlags {
    fn vk_to_raw(src: &VkValidationCacheCreateFlags, dst: &mut RawVkValidationCacheCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkValidationCacheCreateFlags> for RawVkValidationCacheCreateFlags {
    fn vk_to_wrapped(src: &RawVkValidationCacheCreateFlags) -> VkValidationCacheCreateFlags {
        VkValidationCacheCreateFlags {
            
        }
    }
}

impl Default for VkValidationCacheCreateFlags {
    fn default() -> VkValidationCacheCreateFlags {
        VkValidationCacheCreateFlags {
            
        }
    }
}

impl VkValidationCacheCreateFlags {
    
    pub fn none() -> VkValidationCacheCreateFlags {
        VkValidationCacheCreateFlags {
            
        }
    }
    
    pub fn all() -> VkValidationCacheCreateFlags {
        VkValidationCacheCreateFlags {
            
        }
    }
}

impl VkValidationCacheCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}