// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineCacheCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCacheCreateFlags {
    
}

impl VkRawType<VkPipelineCacheCreateFlags> for RawVkPipelineCacheCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCacheCreateFlags) -> VkPipelineCacheCreateFlags {
        VkPipelineCacheCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineCacheCreateFlags> for VkPipelineCacheCreateFlags {
    fn vk_to_raw(src: &VkPipelineCacheCreateFlags, dst: &mut RawVkPipelineCacheCreateFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_PIPELINE_CACHE_CREATE_FLAGS : VkPipelineCacheCreateFlags = VkPipelineCacheCreateFlags {
    
};

impl VkDefault for VkPipelineCacheCreateFlags {
    fn vk_default() -> VkPipelineCacheCreateFlags {
        STATIC_VK_PIPELINE_CACHE_CREATE_FLAGS
    }
}