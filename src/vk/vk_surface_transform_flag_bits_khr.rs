// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;

pub type RawVkSurfaceTransformFlagBitsKHR = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSurfaceTransformFlagsKHR {
}

impl VkFrom<VkSurfaceTransformFlagsKHR> for RawVkSurfaceTransformFlagBitsKHR {
    
    fn vk_from(value: &VkSurfaceTransformFlagsKHR) -> Self {
        *value as i32
    }
}

impl VkFrom<RawVkSurfaceTransformFlagBitsKHR> for VkSurfaceTransformFlagsKHR {
    
    fn vk_from(value: &RawVkSurfaceTransformFlagBitsKHR) -> Self {
        unsafe {
            *((value as *const i32) as *const VkSurfaceTransformFlagsKHR)
        }
    }
}