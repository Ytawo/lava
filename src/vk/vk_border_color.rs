// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkBorderColor = i32;

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

impl VkRawType<VkBorderColor> for RawVkBorderColor {
    fn vk_to_wrapped(src: &RawVkBorderColor) -> VkBorderColor {
        unsafe {
            *((src as *const i32) as *const VkBorderColor)
        }
    }
}

impl VkWrappedType<RawVkBorderColor> for VkBorderColor {
    fn vk_to_raw(src: &VkBorderColor, dst: &mut RawVkBorderColor) {
        *dst = *src as i32
    }
}

impl Default for VkBorderColor {
    fn default() -> VkBorderColor {
        VkBorderColor::FloatTransparentBlack
    }
}