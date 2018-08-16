// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkPeerMemoryFeatureFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkPeerMemoryFeatureFlags {
    pub copy_src: bool,
    pub copy_dst: bool,
    pub generic_src: bool,
    pub generic_dst: bool,
}

impl VkRawType<VkPeerMemoryFeatureFlags> for RawVkPeerMemoryFeatureFlags {
    fn vk_to_wrapped(src: &RawVkPeerMemoryFeatureFlags) -> VkPeerMemoryFeatureFlags {
        VkPeerMemoryFeatureFlags {
            copy_src: (src & 0x00000001) != 0,
            copy_dst: (src & 0x00000002) != 0,
            generic_src: (src & 0x00000004) != 0,
            generic_dst: (src & 0x00000008) != 0,
        }
    }
}

impl VkWrappedType<RawVkPeerMemoryFeatureFlags> for VkPeerMemoryFeatureFlags {
    fn vk_to_raw(src: &VkPeerMemoryFeatureFlags, dst: &mut RawVkPeerMemoryFeatureFlags) {
        *dst = 0;
        if src.copy_src { *dst |= 0x00000001; }
        if src.copy_dst { *dst |= 0x00000002; }
        if src.generic_src { *dst |= 0x00000004; }
        if src.generic_dst { *dst |= 0x00000008; }
    }
}

impl Default for VkPeerMemoryFeatureFlags {
    fn default() -> VkPeerMemoryFeatureFlags {
        VkPeerMemoryFeatureFlags {
            copy_src: false,
            copy_dst: false,
            generic_src: false,
            generic_dst: false,
        }
    }
}

impl VkPeerMemoryFeatureFlags {
    
    pub fn none() -> VkPeerMemoryFeatureFlags {
        VkPeerMemoryFeatureFlags {
            copy_src: false,
            copy_dst: false,
            generic_src: false,
            generic_dst: false,
        }
    }
    
    pub fn all() -> VkPeerMemoryFeatureFlags {
        VkPeerMemoryFeatureFlags {
            copy_src: true,
            copy_dst: true,
            generic_src: true,
            generic_dst: true,
        }
    }
}