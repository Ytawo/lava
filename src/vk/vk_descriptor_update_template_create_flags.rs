// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorUpdateTemplateCreateFlags {
    
}

pub type RawVkDescriptorUpdateTemplateCreateFlags = u32;

impl VkWrappedType<RawVkDescriptorUpdateTemplateCreateFlags> for VkDescriptorUpdateTemplateCreateFlags {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplateCreateFlags, dst: &mut RawVkDescriptorUpdateTemplateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDescriptorUpdateTemplateCreateFlags> for RawVkDescriptorUpdateTemplateCreateFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorUpdateTemplateCreateFlags) -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}

impl Default for VkDescriptorUpdateTemplateCreateFlags {
    fn default() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}

impl VkDescriptorUpdateTemplateCreateFlags {
    
    pub fn none() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}

impl VkDescriptorUpdateTemplateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}