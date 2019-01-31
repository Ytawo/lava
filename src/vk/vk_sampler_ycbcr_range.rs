// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerYcbcrRange {
    ItuFull = 0,
    ItuNarrow = 1,
}

pub type RawVkSamplerYcbcrRange = i32;

impl VkWrappedType<RawVkSamplerYcbcrRange> for VkSamplerYcbcrRange {
    fn vk_to_raw(src: &VkSamplerYcbcrRange, dst: &mut RawVkSamplerYcbcrRange) {
        *dst = *src as i32
    }
}

impl VkRawType<VkSamplerYcbcrRange> for RawVkSamplerYcbcrRange {
    fn vk_to_wrapped(src: &RawVkSamplerYcbcrRange) -> VkSamplerYcbcrRange {
        unsafe {
            *((src as *const i32) as *const VkSamplerYcbcrRange)
        }
    }
}

impl Default for VkSamplerYcbcrRange {
    fn default() -> VkSamplerYcbcrRange {
        VkSamplerYcbcrRange::ItuFull
    }
}