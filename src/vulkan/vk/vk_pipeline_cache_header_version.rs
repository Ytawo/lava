// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineCacheHeaderVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheHeaderVersion.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPipelineCacheHeaderVersion {
    One = 1,
}

#[doc(hidden)]
pub type RawVkPipelineCacheHeaderVersion = i32;

impl VkWrappedType<RawVkPipelineCacheHeaderVersion> for VkPipelineCacheHeaderVersion {
    fn vk_to_raw(src: &VkPipelineCacheHeaderVersion, dst: &mut RawVkPipelineCacheHeaderVersion) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPipelineCacheHeaderVersion> for RawVkPipelineCacheHeaderVersion {
    fn vk_to_wrapped(src: &RawVkPipelineCacheHeaderVersion) -> VkPipelineCacheHeaderVersion {
        unsafe {
            *((src as *const i32) as *const VkPipelineCacheHeaderVersion)
        }
    }
}

impl Default for VkPipelineCacheHeaderVersion {
    fn default() -> VkPipelineCacheHeaderVersion {
        VkPipelineCacheHeaderVersion::One
    }
}