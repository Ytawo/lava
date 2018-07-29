// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkColorComponentFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkColorComponentFlags {
    r: bool,
    g: bool,
    b: bool,
    a: bool,
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

impl VkWrappedType<RawVkColorComponentFlags> for VkColorComponentFlags {
    fn vk_to_raw(src: &VkColorComponentFlags, dst: &mut RawVkColorComponentFlags) {
        *dst = 0;
        if src.r { *dst |= 0x00000001; }
        if src.g { *dst |= 0x00000002; }
        if src.b { *dst |= 0x00000004; }
        if src.a { *dst |= 0x00000008; }
    }
}

pub static STATIC_VK_COLOR_COMPONENT_FLAGS : VkColorComponentFlags = VkColorComponentFlags {
    r: false,
    g: false,
    b: false,
    a: false,
};

impl VkDefault for VkColorComponentFlags {
    fn vk_default() -> VkColorComponentFlags {
        STATIC_VK_COLOR_COMPONENT_FLAGS
    }
}