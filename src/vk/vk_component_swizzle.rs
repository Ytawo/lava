// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkComponentSwizzle {
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}

pub type RawVkComponentSwizzle = i32;

impl VkWrappedType<RawVkComponentSwizzle> for VkComponentSwizzle {
    fn vk_to_raw(src: &VkComponentSwizzle, dst: &mut RawVkComponentSwizzle) {
        *dst = *src as i32
    }
}

impl VkRawType<VkComponentSwizzle> for RawVkComponentSwizzle {
    fn vk_to_wrapped(src: &RawVkComponentSwizzle) -> VkComponentSwizzle {
        unsafe {
            *((src as *const i32) as *const VkComponentSwizzle)
        }
    }
}

impl Default for VkComponentSwizzle {
    fn default() -> VkComponentSwizzle {
        VkComponentSwizzle::Identity
    }
}