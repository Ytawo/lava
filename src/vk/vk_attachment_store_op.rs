// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkAttachmentStoreOp = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkAttachmentStoreOp {
    Store = 0,
    DontCare = 1,
}

impl VkRawType<VkAttachmentStoreOp> for RawVkAttachmentStoreOp {
    fn vk_to_wrapped(src: &RawVkAttachmentStoreOp) -> VkAttachmentStoreOp {
        unsafe {
            *((src as *const i32) as *const VkAttachmentStoreOp)
        }
    }
}

impl VkWrappedType<RawVkAttachmentStoreOp> for VkAttachmentStoreOp {
    fn vk_to_raw(src: &VkAttachmentStoreOp, dst: &mut RawVkAttachmentStoreOp) {
        *dst = *src as i32
    }
}

impl Default for VkAttachmentStoreOp {
    fn default() -> VkAttachmentStoreOp {
        VkAttachmentStoreOp::Store
    }
}