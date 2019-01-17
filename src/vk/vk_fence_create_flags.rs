// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkFenceCreateFlags {
    pub signaled: bool,
}

pub type RawVkFenceCreateFlags = u32;

impl VkWrappedType<RawVkFenceCreateFlags> for VkFenceCreateFlags {
    fn vk_to_raw(src: &VkFenceCreateFlags, dst: &mut RawVkFenceCreateFlags) {
        *dst = 0;
        if src.signaled { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkFenceCreateFlags> for RawVkFenceCreateFlags {
    fn vk_to_wrapped(src: &RawVkFenceCreateFlags) -> VkFenceCreateFlags {
        VkFenceCreateFlags {
            signaled: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkFenceCreateFlags {
    fn default() -> VkFenceCreateFlags {
        VkFenceCreateFlags {
            signaled: false,
        }
    }
}

impl VkFenceCreateFlags {
    
    pub fn none() -> VkFenceCreateFlags {
        VkFenceCreateFlags {
            signaled: false,
        }
    }
    
    pub fn all() -> VkFenceCreateFlags {
        VkFenceCreateFlags {
            signaled: true,
        }
    }
}

impl VkFenceCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.signaled { 0x00000001 } else { 0 }
    }
}