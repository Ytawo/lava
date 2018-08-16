// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkCullModeFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkCullModeFlags {
    pub front: bool,
    pub back: bool,
    pub front_and_back: bool,
}

impl VkRawType<VkCullModeFlags> for RawVkCullModeFlags {
    fn vk_to_wrapped(src: &RawVkCullModeFlags) -> VkCullModeFlags {
        VkCullModeFlags {
            front: (src & 0x00000001) != 0,
            back: (src & 0x00000002) != 0,
            front_and_back: (src & 0x00000003) != 0,
        }
    }
}

impl VkWrappedType<RawVkCullModeFlags> for VkCullModeFlags {
    fn vk_to_raw(src: &VkCullModeFlags, dst: &mut RawVkCullModeFlags) {
        *dst = 0;
        if src.front { *dst |= 0x00000001; }
        if src.back { *dst |= 0x00000002; }
        if src.front_and_back { *dst |= 0x00000003; }
    }
}

impl Default for VkCullModeFlags {
    fn default() -> VkCullModeFlags {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
}

impl VkCullModeFlags {
    
    pub fn none() -> VkCullModeFlags {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
    
    pub fn all() -> VkCullModeFlags {
        VkCullModeFlags {
            front: true,
            back: true,
            front_and_back: true,
        }
    }
}