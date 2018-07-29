// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkFilter = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkFilter {
    Nearest = 0,
    Linear = 1,
    CubicImg = 1000015000,
}

impl VkRawType<VkFilter> for RawVkFilter {
    fn vk_to_wrapped(src: &RawVkFilter) -> VkFilter {
        unsafe {
            *((src as *const i32) as *const VkFilter)
        }
    }
}

impl VkWrappedType<RawVkFilter> for VkFilter {
    fn vk_to_raw(src: &VkFilter, dst: &mut RawVkFilter) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_FILTER : VkFilter = VkFilter::Nearest;

impl VkDefault for VkFilter {
    fn vk_default() -> VkFilter {
        STATIC_VK_FILTER
    }
}