// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkVendorId = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkVendorId {
    Viv = 0x10001,
    Vsi = 0x10002,
    Kazan = 0x10003,
}

impl VkRawType<VkVendorId> for RawVkVendorId {
    fn vk_to_wrapped(src: &RawVkVendorId) -> VkVendorId {
        unsafe {
            *((src as *const i32) as *const VkVendorId)
        }
    }
}

impl VkWrappedType<RawVkVendorId> for VkVendorId {
    fn vk_to_raw(src: &VkVendorId, dst: &mut RawVkVendorId) {
        *dst = *src as i32
    }
}

impl Default for VkVendorId {
    fn default() -> VkVendorId {
        VkVendorId::Viv
    }
}