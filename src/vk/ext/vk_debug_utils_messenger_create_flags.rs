// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkDebugUtilsMessengerCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkDebugUtilsMessengerCreateFlags {
    
}

impl VkRawType<VkDebugUtilsMessengerCreateFlags> for RawVkDebugUtilsMessengerCreateFlags {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessengerCreateFlags) -> VkDebugUtilsMessengerCreateFlags {
        VkDebugUtilsMessengerCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkDebugUtilsMessengerCreateFlags> for VkDebugUtilsMessengerCreateFlags {
    fn vk_to_raw(src: &VkDebugUtilsMessengerCreateFlags, dst: &mut RawVkDebugUtilsMessengerCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkDebugUtilsMessengerCreateFlags {
    fn default() -> VkDebugUtilsMessengerCreateFlags {
        VkDebugUtilsMessengerCreateFlags {
            
        }
    }
}

impl VkDebugUtilsMessengerCreateFlags {
    
    pub fn none() -> VkDebugUtilsMessengerCreateFlags {
        VkDebugUtilsMessengerCreateFlags {
            
        }
    }
    
    pub fn all() -> VkDebugUtilsMessengerCreateFlags {
        VkDebugUtilsMessengerCreateFlags {
            
        }
    }
}