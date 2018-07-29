// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDisplayEventType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDisplayEventType {
    FirstPixelOut = 0,
}

impl VkRawType<VkDisplayEventType> for RawVkDisplayEventType {
    fn vk_to_wrapped(src: &RawVkDisplayEventType) -> VkDisplayEventType {
        unsafe {
            *((src as *const i32) as *const VkDisplayEventType)
        }
    }
}

impl VkWrappedType<RawVkDisplayEventType> for VkDisplayEventType {
    fn vk_to_raw(src: &VkDisplayEventType, dst: &mut RawVkDisplayEventType) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_DISPLAY_EVENT_TYPE : VkDisplayEventType = VkDisplayEventType::FirstPixelOut;

impl VkDefault for VkDisplayEventType {
    fn vk_default() -> VkDisplayEventType {
        STATIC_VK_DISPLAY_EVENT_TYPE
    }
}