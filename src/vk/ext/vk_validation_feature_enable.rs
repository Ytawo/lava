// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkValidationFeatureEnable {
    GpuAssisted = 0,
    GpuAssistedReserveBindingSlot = 1,
}

pub type RawVkValidationFeatureEnable = i32;

impl VkWrappedType<RawVkValidationFeatureEnable> for VkValidationFeatureEnable {
    fn vk_to_raw(src: &VkValidationFeatureEnable, dst: &mut RawVkValidationFeatureEnable) {
        *dst = *src as i32
    }
}

impl VkRawType<VkValidationFeatureEnable> for RawVkValidationFeatureEnable {
    fn vk_to_wrapped(src: &RawVkValidationFeatureEnable) -> VkValidationFeatureEnable {
        unsafe {
            *((src as *const i32) as *const VkValidationFeatureEnable)
        }
    }
}

impl Default for VkValidationFeatureEnable {
    fn default() -> VkValidationFeatureEnable {
        VkValidationFeatureEnable::GpuAssisted
    }
}