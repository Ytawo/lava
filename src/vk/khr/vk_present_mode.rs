// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPresentMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPresentMode {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    SharedDemandRefresh = 1000111000,
    SharedContinuousRefresh = 1000111001,
}

impl VkRawType<VkPresentMode> for RawVkPresentMode {
    fn vk_to_wrapped(src: &RawVkPresentMode) -> VkPresentMode {
        unsafe {
            *((src as *const i32) as *const VkPresentMode)
        }
    }
}

impl VkWrappedType<RawVkPresentMode> for VkPresentMode {
    fn vk_to_raw(src: &VkPresentMode, dst: &mut RawVkPresentMode) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_PRESENT_MODE : VkPresentMode = VkPresentMode::Immediate;

impl VkDefault for VkPresentMode {
    fn vk_default() -> VkPresentMode {
        STATIC_VK_PRESENT_MODE
    }
}