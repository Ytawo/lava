// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkRasterizationOrder = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkRasterizationOrder {
    Strict = 0,
    Relaxed = 1,
}

impl VkRawType<VkRasterizationOrder> for RawVkRasterizationOrder {
    fn vk_to_wrapped(src: &RawVkRasterizationOrder) -> VkRasterizationOrder {
        unsafe {
            *((src as *const i32) as *const VkRasterizationOrder)
        }
    }
}

impl VkWrappedType<RawVkRasterizationOrder> for VkRasterizationOrder {
    fn vk_to_raw(src: &VkRasterizationOrder, dst: &mut RawVkRasterizationOrder) {
        *dst = *src as i32
    }
}

impl Default for VkRasterizationOrder {
    fn default() -> VkRasterizationOrder {
        VkRasterizationOrder::Strict
    }
}