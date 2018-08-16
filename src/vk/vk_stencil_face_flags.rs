// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkStencilFaceFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkStencilFaceFlags {
    pub front: bool,
    pub back: bool,
    pub _and_back: bool,
}

impl VkRawType<VkStencilFaceFlags> for RawVkStencilFaceFlags {
    fn vk_to_wrapped(src: &RawVkStencilFaceFlags) -> VkStencilFaceFlags {
        VkStencilFaceFlags {
            front: (src & 0x00000001) != 0,
            back: (src & 0x00000002) != 0,
            _and_back: (src & 0x00000003) != 0,
        }
    }
}

impl VkWrappedType<RawVkStencilFaceFlags> for VkStencilFaceFlags {
    fn vk_to_raw(src: &VkStencilFaceFlags, dst: &mut RawVkStencilFaceFlags) {
        *dst = 0;
        if src.front { *dst |= 0x00000001; }
        if src.back { *dst |= 0x00000002; }
        if src._and_back { *dst |= 0x00000003; }
    }
}

impl Default for VkStencilFaceFlags {
    fn default() -> VkStencilFaceFlags {
        VkStencilFaceFlags {
            front: false,
            back: false,
            _and_back: false,
        }
    }
}

impl VkStencilFaceFlags {
    
    pub fn none() -> VkStencilFaceFlags {
        VkStencilFaceFlags {
            front: false,
            back: false,
            _and_back: false,
        }
    }
    
    pub fn all() -> VkStencilFaceFlags {
        VkStencilFaceFlags {
            front: true,
            back: true,
            _and_back: true,
        }
    }
}