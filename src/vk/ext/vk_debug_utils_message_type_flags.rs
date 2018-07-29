// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDebugUtilsMessageTypeFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkDebugUtilsMessageTypeFlags {
    general: bool,
    validation: bool,
    performance: bool,
}

impl VkRawType<VkDebugUtilsMessageTypeFlags> for RawVkDebugUtilsMessageTypeFlags {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessageTypeFlags) -> VkDebugUtilsMessageTypeFlags {
        VkDebugUtilsMessageTypeFlags {
            general: (src & 0x00000001) != 0,
            validation: (src & 0x00000002) != 0,
            performance: (src & 0x00000004) != 0,
        }
    }
}

impl VkWrappedType<RawVkDebugUtilsMessageTypeFlags> for VkDebugUtilsMessageTypeFlags {
    fn vk_to_raw(src: &VkDebugUtilsMessageTypeFlags, dst: &mut RawVkDebugUtilsMessageTypeFlags) {
        *dst = 0;
        if src.general { *dst |= 0x00000001; }
        if src.validation { *dst |= 0x00000002; }
        if src.performance { *dst |= 0x00000004; }
    }
}

pub static STATIC_VK_DEBUG_UTILS_MESSAGE_TYPE_FLAGS : VkDebugUtilsMessageTypeFlags = VkDebugUtilsMessageTypeFlags {
    general: false,
    validation: false,
    performance: false,
};

impl VkDefault for VkDebugUtilsMessageTypeFlags {
    fn vk_default() -> VkDebugUtilsMessageTypeFlags {
        STATIC_VK_DEBUG_UTILS_MESSAGE_TYPE_FLAGS
    }
}