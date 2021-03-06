// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkMemoryOverallocationBehaviorAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkMemoryOverallocationBehavior {
    Default = 0,
    Allowed = 1,
    Disallowed = 2,
}

#[doc(hidden)]
pub type RawVkMemoryOverallocationBehavior = i32;

impl VkWrappedType<RawVkMemoryOverallocationBehavior> for VkMemoryOverallocationBehavior {
    fn vk_to_raw(src: &VkMemoryOverallocationBehavior, dst: &mut RawVkMemoryOverallocationBehavior) {
        *dst = *src as i32
    }
}

impl VkRawType<VkMemoryOverallocationBehavior> for RawVkMemoryOverallocationBehavior {
    fn vk_to_wrapped(src: &RawVkMemoryOverallocationBehavior) -> VkMemoryOverallocationBehavior {
        unsafe {
            *((src as *const i32) as *const VkMemoryOverallocationBehavior)
        }
    }
}

impl Default for VkMemoryOverallocationBehavior {
    fn default() -> VkMemoryOverallocationBehavior {
        VkMemoryOverallocationBehavior::Default
    }
}