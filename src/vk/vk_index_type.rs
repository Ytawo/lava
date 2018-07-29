// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkIndexType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkIndexType {
    Uint16 = 0,
    Uint32 = 1,
}

impl VkRawType<VkIndexType> for RawVkIndexType {
    fn vk_to_wrapped(src: &RawVkIndexType) -> VkIndexType {
        unsafe {
            *((src as *const i32) as *const VkIndexType)
        }
    }
}

impl VkWrappedType<RawVkIndexType> for VkIndexType {
    fn vk_to_raw(src: &VkIndexType, dst: &mut RawVkIndexType) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_INDEX_TYPE : VkIndexType = VkIndexType::Uint16;

impl VkDefault for VkIndexType {
    fn vk_default() -> VkIndexType {
        STATIC_VK_INDEX_TYPE
    }
}