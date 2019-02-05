// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPresentModeKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPresentModeKHR.html).
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

#[doc(hidden)]
pub type RawVkPresentMode = i32;

impl VkWrappedType<RawVkPresentMode> for VkPresentMode {
    fn vk_to_raw(src: &VkPresentMode, dst: &mut RawVkPresentMode) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPresentMode> for RawVkPresentMode {
    fn vk_to_wrapped(src: &RawVkPresentMode) -> VkPresentMode {
        unsafe {
            *((src as *const i32) as *const VkPresentMode)
        }
    }
}

impl Default for VkPresentMode {
    fn default() -> VkPresentMode {
        VkPresentMode::Immediate
    }
}