// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkImageType {
    _1d = 0,
    _2d = 1,
    _3d = 2,
}

pub type RawVkImageType = i32;

impl VkWrappedType<RawVkImageType> for VkImageType {
    fn vk_to_raw(src: &VkImageType, dst: &mut RawVkImageType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkImageType> for RawVkImageType {
    fn vk_to_wrapped(src: &RawVkImageType) -> VkImageType {
        unsafe {
            *((src as *const i32) as *const VkImageType)
        }
    }
}

impl Default for VkImageType {
    fn default() -> VkImageType {
        VkImageType::_1d
    }
}