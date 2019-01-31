// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkBlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    DstColor = 4,
    OneMinusDstColor = 5,
    SrcAlpha = 6,
    OneMinusSrcAlpha = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
    ConstantColor = 10,
    OneMinusConstantColor = 11,
    ConstantAlpha = 12,
    OneMinusConstantAlpha = 13,
    SrcAlphaSaturate = 14,
    Src1Color = 15,
    OneMinusSrc1Color = 16,
    Src1Alpha = 17,
    OneMinusSrc1Alpha = 18,
}

pub type RawVkBlendFactor = i32;

impl VkWrappedType<RawVkBlendFactor> for VkBlendFactor {
    fn vk_to_raw(src: &VkBlendFactor, dst: &mut RawVkBlendFactor) {
        *dst = *src as i32
    }
}

impl VkRawType<VkBlendFactor> for RawVkBlendFactor {
    fn vk_to_wrapped(src: &RawVkBlendFactor) -> VkBlendFactor {
        unsafe {
            *((src as *const i32) as *const VkBlendFactor)
        }
    }
}

impl Default for VkBlendFactor {
    fn default() -> VkBlendFactor {
        VkBlendFactor::Zero
    }
}