// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPointClippingBehavior](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPointClippingBehavior.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPointClippingBehavior {
    AllClipPlanes = 0,
    UserClipPlanesOnly = 1,
}

#[doc(hidden)]
pub type RawVkPointClippingBehavior = i32;

impl VkWrappedType<RawVkPointClippingBehavior> for VkPointClippingBehavior {
    fn vk_to_raw(src: &VkPointClippingBehavior, dst: &mut RawVkPointClippingBehavior) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPointClippingBehavior> for RawVkPointClippingBehavior {
    fn vk_to_wrapped(src: &RawVkPointClippingBehavior) -> VkPointClippingBehavior {
        unsafe {
            *((src as *const i32) as *const VkPointClippingBehavior)
        }
    }
}

impl Default for VkPointClippingBehavior {
    fn default() -> VkPointClippingBehavior {
        VkPointClippingBehavior::AllClipPlanes
    }
}