// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSamplerCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkSamplerCreateFlags {
    
}

impl VkRawType<VkSamplerCreateFlags> for RawVkSamplerCreateFlags {
    fn vk_to_wrapped(src: &RawVkSamplerCreateFlags) -> VkSamplerCreateFlags {
        VkSamplerCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkSamplerCreateFlags> for VkSamplerCreateFlags {
    fn vk_to_raw(src: &VkSamplerCreateFlags, dst: &mut RawVkSamplerCreateFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_SAMPLER_CREATE_FLAGS : VkSamplerCreateFlags = VkSamplerCreateFlags {
    
};

impl VkDefault for VkSamplerCreateFlags {
    fn vk_default() -> VkSamplerCreateFlags {
        STATIC_VK_SAMPLER_CREATE_FLAGS
    }
}