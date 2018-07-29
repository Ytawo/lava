// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkImageTiling = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkImageTiling {
    Optimal = 0,
    Linear = 1,
}

impl VkRawType<VkImageTiling> for RawVkImageTiling {
    fn vk_to_wrapped(src: &RawVkImageTiling) -> VkImageTiling {
        unsafe {
            *((src as *const i32) as *const VkImageTiling)
        }
    }
}

impl VkWrappedType<RawVkImageTiling> for VkImageTiling {
    fn vk_to_raw(src: &VkImageTiling, dst: &mut RawVkImageTiling) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_IMAGE_TILING : VkImageTiling = VkImageTiling::Optimal;

impl VkDefault for VkImageTiling {
    fn vk_default() -> VkImageTiling {
        STATIC_VK_IMAGE_TILING
    }
}