// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkAccelerationStructureTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkAccelerationStructureType {
    TopLevel = 0,
    BottomLevel = 1,
}

#[doc(hidden)]
pub type RawVkAccelerationStructureType = i32;

impl VkWrappedType<RawVkAccelerationStructureType> for VkAccelerationStructureType {
    fn vk_to_raw(src: &VkAccelerationStructureType, dst: &mut RawVkAccelerationStructureType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkAccelerationStructureType> for RawVkAccelerationStructureType {
    fn vk_to_wrapped(src: &RawVkAccelerationStructureType) -> VkAccelerationStructureType {
        unsafe {
            *((src as *const i32) as *const VkAccelerationStructureType)
        }
    }
}

impl Default for VkAccelerationStructureType {
    fn default() -> VkAccelerationStructureType {
        VkAccelerationStructureType::TopLevel
    }
}