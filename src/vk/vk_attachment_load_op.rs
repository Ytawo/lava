// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkAttachmentLoadOp = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkAttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2,
}

impl VkType<RawVkAttachmentLoadOp> for VkAttachmentLoadOp {
    
    fn vk_to_raw(src: &VkAttachmentLoadOp, dst: &mut RawVkAttachmentLoadOp) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkAttachmentLoadOp) -> VkAttachmentLoadOp {
        unsafe {
            *((src as *const i32) as *const VkAttachmentLoadOp)
        }
    }
    
    fn vk_default() -> VkAttachmentLoadOp {
        VkAttachmentLoadOp::Load
    }
}