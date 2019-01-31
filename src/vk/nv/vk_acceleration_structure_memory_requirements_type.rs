// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkAccelerationStructureMemoryRequirementsType {
    Object = 0,
    BuildScratch = 1,
    UpdateScratch = 2,
}

pub type RawVkAccelerationStructureMemoryRequirementsType = i32;

impl VkWrappedType<RawVkAccelerationStructureMemoryRequirementsType> for VkAccelerationStructureMemoryRequirementsType {
    fn vk_to_raw(src: &VkAccelerationStructureMemoryRequirementsType, dst: &mut RawVkAccelerationStructureMemoryRequirementsType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkAccelerationStructureMemoryRequirementsType> for RawVkAccelerationStructureMemoryRequirementsType {
    fn vk_to_wrapped(src: &RawVkAccelerationStructureMemoryRequirementsType) -> VkAccelerationStructureMemoryRequirementsType {
        unsafe {
            *((src as *const i32) as *const VkAccelerationStructureMemoryRequirementsType)
        }
    }
}

impl Default for VkAccelerationStructureMemoryRequirementsType {
    fn default() -> VkAccelerationStructureMemoryRequirementsType {
        VkAccelerationStructureMemoryRequirementsType::Object
    }
}