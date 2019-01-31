// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkColorComponentFlags {
    pub r: bool,
    pub g: bool,
    pub b: bool,
    pub a: bool,
}

pub type RawVkColorComponentFlags = u32;

impl VkWrappedType<RawVkColorComponentFlags> for VkColorComponentFlags {
    fn vk_to_raw(src: &VkColorComponentFlags, dst: &mut RawVkColorComponentFlags) {
        *dst = 0;
        if src.r { *dst |= 0x00000001; }
        if src.g { *dst |= 0x00000002; }
        if src.b { *dst |= 0x00000004; }
        if src.a { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkColorComponentFlags> for RawVkColorComponentFlags {
    fn vk_to_wrapped(src: &RawVkColorComponentFlags) -> VkColorComponentFlags {
        VkColorComponentFlags {
            r: (src & 0x00000001) != 0,
            g: (src & 0x00000002) != 0,
            b: (src & 0x00000004) != 0,
            a: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkColorComponentFlags {
    fn default() -> VkColorComponentFlags {
        VkColorComponentFlags {
            r: false,
            g: false,
            b: false,
            a: false,
        }
    }
}

impl VkColorComponentFlags {
    
    pub fn none() -> VkColorComponentFlags {
        VkColorComponentFlags {
            r: false,
            g: false,
            b: false,
            a: false,
        }
    }
    
    pub fn all() -> VkColorComponentFlags {
        VkColorComponentFlags {
            r: true,
            g: true,
            b: true,
            a: true,
        }
    }
}

#[macro_export]
macro_rules! VkColorComponentFlags {
    ( $( $x:ident ),* ) => {
        VkColorComponentFlags {
            $($x: true,)*
            ..VkColorComponentFlags::none()
        }
    }
}

impl VkColorComponentFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.r { 0x00000001 } else { 0 }
        + if self.g { 0x00000002 } else { 0 }
        + if self.b { 0x00000004 } else { 0 }
        + if self.a { 0x00000008 } else { 0 }
    }
}