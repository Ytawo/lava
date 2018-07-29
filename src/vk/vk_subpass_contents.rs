// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSubpassContents = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSubpassContents {
    Inline = 0,
    SecondaryCommandBuffers = 1,
}

impl VkRawType<VkSubpassContents> for RawVkSubpassContents {
    fn vk_to_wrapped(src: &RawVkSubpassContents) -> VkSubpassContents {
        unsafe {
            *((src as *const i32) as *const VkSubpassContents)
        }
    }
}

impl VkWrappedType<RawVkSubpassContents> for VkSubpassContents {
    fn vk_to_raw(src: &VkSubpassContents, dst: &mut RawVkSubpassContents) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_SUBPASS_CONTENTS : VkSubpassContents = VkSubpassContents::Inline;

impl VkDefault for VkSubpassContents {
    fn vk_default() -> VkSubpassContents {
        STATIC_VK_SUBPASS_CONTENTS
    }
}