// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkSwapchainCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkSwapchainCreateFlags {
    pub split_instance_bind_regions: bool,
    pub protected: bool,
}

impl VkRawType<VkSwapchainCreateFlags> for RawVkSwapchainCreateFlags {
    fn vk_to_wrapped(src: &RawVkSwapchainCreateFlags) -> VkSwapchainCreateFlags {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: (src & 0x00000001) != 0,
            protected: (src & 0x00000002) != 0,
        }
    }
}

impl VkWrappedType<RawVkSwapchainCreateFlags> for VkSwapchainCreateFlags {
    fn vk_to_raw(src: &VkSwapchainCreateFlags, dst: &mut RawVkSwapchainCreateFlags) {
        *dst = 0;
        if src.split_instance_bind_regions { *dst |= 0x00000001; }
        if src.protected { *dst |= 0x00000002; }
    }
}

impl Default for VkSwapchainCreateFlags {
    fn default() -> VkSwapchainCreateFlags {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: false,
            protected: false,
        }
    }
}

impl VkSwapchainCreateFlags {
    
    pub fn none() -> VkSwapchainCreateFlags {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: false,
            protected: false,
        }
    }
    
    pub fn all() -> VkSwapchainCreateFlags {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: true,
            protected: true,
        }
    }
}