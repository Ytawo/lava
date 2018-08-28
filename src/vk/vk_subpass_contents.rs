// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

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

impl Default for VkSubpassContents {
    fn default() -> VkSubpassContents {
        VkSubpassContents::Inline
    }
}