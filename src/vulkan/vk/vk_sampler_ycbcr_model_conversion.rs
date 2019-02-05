// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSamplerYcbcrModelConversion](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSamplerYcbcrModelConversion.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerYcbcrModelConversion {
    RgbIdentity = 0,
    YcbcrIdentity = 1,
    Ycbcr709 = 2,
    Ycbcr601 = 3,
    Ycbcr2020 = 4,
}

#[doc(hidden)]
pub type RawVkSamplerYcbcrModelConversion = i32;

impl VkWrappedType<RawVkSamplerYcbcrModelConversion> for VkSamplerYcbcrModelConversion {
    fn vk_to_raw(src: &VkSamplerYcbcrModelConversion, dst: &mut RawVkSamplerYcbcrModelConversion) {
        *dst = *src as i32
    }
}

impl VkRawType<VkSamplerYcbcrModelConversion> for RawVkSamplerYcbcrModelConversion {
    fn vk_to_wrapped(src: &RawVkSamplerYcbcrModelConversion) -> VkSamplerYcbcrModelConversion {
        unsafe {
            *((src as *const i32) as *const VkSamplerYcbcrModelConversion)
        }
    }
}

impl Default for VkSamplerYcbcrModelConversion {
    fn default() -> VkSamplerYcbcrModelConversion {
        VkSamplerYcbcrModelConversion::RgbIdentity
    }
}