// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkSurfaceCounterFlags {
    pub vblank: bool,
}

pub type RawVkSurfaceCounterFlags = u32;

impl VkWrappedType<RawVkSurfaceCounterFlags> for VkSurfaceCounterFlags {
    fn vk_to_raw(src: &VkSurfaceCounterFlags, dst: &mut RawVkSurfaceCounterFlags) {
        *dst = 0;
        if src.vblank { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkSurfaceCounterFlags> for RawVkSurfaceCounterFlags {
    fn vk_to_wrapped(src: &RawVkSurfaceCounterFlags) -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkSurfaceCounterFlags {
    fn default() -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: false,
        }
    }
}

impl VkSurfaceCounterFlags {
    
    pub fn none() -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: false,
        }
    }
    
    pub fn all() -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: true,
        }
    }
}

#[macro_export]
macro_rules! VkSurfaceCounterFlags {
    ( $( $x:ident ),* ) => {
        VkSurfaceCounterFlags {
            $($x: true,)*
            ..VkSurfaceCounterFlags::none()
        }
    }
}

impl VkSurfaceCounterFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.vblank { 0x00000001 } else { 0 }
    }
}