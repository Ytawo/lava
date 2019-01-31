// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkBorderColor {
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,
    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5,
}

pub type RawVkBorderColor = i32;

impl VkWrappedType<RawVkBorderColor> for VkBorderColor {
    fn vk_to_raw(src: &VkBorderColor, dst: &mut RawVkBorderColor) {
        *dst = *src as i32
    }
}

impl VkRawType<VkBorderColor> for RawVkBorderColor {
    fn vk_to_wrapped(src: &RawVkBorderColor) -> VkBorderColor {
        unsafe {
            *((src as *const i32) as *const VkBorderColor)
        }
    }
}

impl Default for VkBorderColor {
    fn default() -> VkBorderColor {
        VkBorderColor::FloatTransparentBlack
    }
}