// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerMipmapMode {
    Nearest = 0,
    Linear = 1,
}

pub type RawVkSamplerMipmapMode = i32;

impl VkWrappedType<RawVkSamplerMipmapMode> for VkSamplerMipmapMode {
    fn vk_to_raw(src: &VkSamplerMipmapMode, dst: &mut RawVkSamplerMipmapMode) {
        *dst = *src as i32
    }
}

impl VkRawType<VkSamplerMipmapMode> for RawVkSamplerMipmapMode {
    fn vk_to_wrapped(src: &RawVkSamplerMipmapMode) -> VkSamplerMipmapMode {
        unsafe {
            *((src as *const i32) as *const VkSamplerMipmapMode)
        }
    }
}

impl Default for VkSamplerMipmapMode {
    fn default() -> VkSamplerMipmapMode {
        VkSamplerMipmapMode::Nearest
    }
}