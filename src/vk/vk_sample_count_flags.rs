// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSampleCountFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkSampleCountFlags {
    _1: bool,
    _2: bool,
    _4: bool,
    _8: bool,
    _16: bool,
    _32: bool,
    _64: bool,
}

impl VkRawType<VkSampleCountFlags> for RawVkSampleCountFlags {
    fn vk_to_wrapped(src: &RawVkSampleCountFlags) -> VkSampleCountFlags {
        VkSampleCountFlags {
            _1: (src & 0x00000001) != 0,
            _2: (src & 0x00000002) != 0,
            _4: (src & 0x00000004) != 0,
            _8: (src & 0x00000008) != 0,
            _16: (src & 0x00000010) != 0,
            _32: (src & 0x00000020) != 0,
            _64: (src & 0x00000040) != 0,
        }
    }
}

impl VkWrappedType<RawVkSampleCountFlags> for VkSampleCountFlags {
    fn vk_to_raw(src: &VkSampleCountFlags, dst: &mut RawVkSampleCountFlags) {
        *dst = 0;
        if src._1 { *dst |= 0x00000001; }
        if src._2 { *dst |= 0x00000002; }
        if src._4 { *dst |= 0x00000004; }
        if src._8 { *dst |= 0x00000008; }
        if src._16 { *dst |= 0x00000010; }
        if src._32 { *dst |= 0x00000020; }
        if src._64 { *dst |= 0x00000040; }
    }
}

pub static STATIC_VK_SAMPLE_COUNT_FLAGS : VkSampleCountFlags = VkSampleCountFlags {
    _1: false,
    _2: false,
    _4: false,
    _8: false,
    _16: false,
    _32: false,
    _64: false,
};

impl VkDefault for VkSampleCountFlags {
    fn vk_default() -> VkSampleCountFlags {
        STATIC_VK_SAMPLE_COUNT_FLAGS
    }
}