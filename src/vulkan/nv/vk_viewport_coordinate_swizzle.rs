// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkViewportCoordinateSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportCoordinateSwizzleNV.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkViewportCoordinateSwizzle {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
    PositiveW = 6,
    NegativeW = 7,
}

#[doc(hidden)]
pub type RawVkViewportCoordinateSwizzle = i32;

impl VkWrappedType<RawVkViewportCoordinateSwizzle> for VkViewportCoordinateSwizzle {
    fn vk_to_raw(src: &VkViewportCoordinateSwizzle, dst: &mut RawVkViewportCoordinateSwizzle) {
        *dst = *src as i32
    }
}

impl VkRawType<VkViewportCoordinateSwizzle> for RawVkViewportCoordinateSwizzle {
    fn vk_to_wrapped(src: &RawVkViewportCoordinateSwizzle) -> VkViewportCoordinateSwizzle {
        unsafe {
            *((src as *const i32) as *const VkViewportCoordinateSwizzle)
        }
    }
}

impl Default for VkViewportCoordinateSwizzle {
    fn default() -> VkViewportCoordinateSwizzle {
        VkViewportCoordinateSwizzle::PositiveX
    }
}