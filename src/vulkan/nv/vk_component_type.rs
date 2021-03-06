// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkComponentTypeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentTypeNV.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkComponentType {
    Float16 = 0,
    Float32 = 1,
    Float64 = 2,
    Sint8 = 3,
    Sint16 = 4,
    Sint32 = 5,
    Sint64 = 6,
    Uint8 = 7,
    Uint16 = 8,
    Uint32 = 9,
    Uint64 = 10,
}

#[doc(hidden)]
pub type RawVkComponentType = i32;

impl VkWrappedType<RawVkComponentType> for VkComponentType {
    fn vk_to_raw(src: &VkComponentType, dst: &mut RawVkComponentType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkComponentType> for RawVkComponentType {
    fn vk_to_wrapped(src: &RawVkComponentType) -> VkComponentType {
        unsafe {
            *((src as *const i32) as *const VkComponentType)
        }
    }
}

impl Default for VkComponentType {
    fn default() -> VkComponentType {
        VkComponentType::Float16
    }
}