// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkCompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7,
}

pub type RawVkCompareOp = i32;

impl VkWrappedType<RawVkCompareOp> for VkCompareOp {
    fn vk_to_raw(src: &VkCompareOp, dst: &mut RawVkCompareOp) {
        *dst = *src as i32
    }
}

impl VkRawType<VkCompareOp> for RawVkCompareOp {
    fn vk_to_wrapped(src: &RawVkCompareOp) -> VkCompareOp {
        unsafe {
            *((src as *const i32) as *const VkCompareOp)
        }
    }
}

impl Default for VkCompareOp {
    fn default() -> VkCompareOp {
        VkCompareOp::Never
    }
}