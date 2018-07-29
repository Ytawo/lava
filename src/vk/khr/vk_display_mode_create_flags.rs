// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDisplayModeCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkDisplayModeCreateFlags {
    
}

impl VkRawType<VkDisplayModeCreateFlags> for RawVkDisplayModeCreateFlags {
    fn vk_to_wrapped(src: &RawVkDisplayModeCreateFlags) -> VkDisplayModeCreateFlags {
        VkDisplayModeCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkDisplayModeCreateFlags> for VkDisplayModeCreateFlags {
    fn vk_to_raw(src: &VkDisplayModeCreateFlags, dst: &mut RawVkDisplayModeCreateFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_DISPLAY_MODE_CREATE_FLAGS : VkDisplayModeCreateFlags = VkDisplayModeCreateFlags {
    
};

impl VkDefault for VkDisplayModeCreateFlags {
    fn vk_default() -> VkDisplayModeCreateFlags {
        STATIC_VK_DISPLAY_MODE_CREATE_FLAGS
    }
}