// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSharingMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSharingMode {
    Exclusive = 0,
    Concurrent = 1,
}

impl VkRawType<VkSharingMode> for RawVkSharingMode {
    
    fn vk_to_wrapped(src: &RawVkSharingMode) -> VkSharingMode {
        unsafe {
            *((src as *const i32) as *const VkSharingMode)
        }
    }
}

impl VkWrappedType<RawVkSharingMode> for VkSharingMode {
    
    fn vk_to_raw(src: &VkSharingMode, dst: &mut RawVkSharingMode) {
        *dst = *src as i32
    }
}

impl VkDefault for VkSharingMode {
    
    fn vk_default() -> VkSharingMode {
        VkSharingMode::Exclusive
    }
}