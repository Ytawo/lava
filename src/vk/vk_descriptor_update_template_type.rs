// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDescriptorUpdateTemplateType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDescriptorUpdateTemplateType {
    DescriptorSet = 0,
    PushDescriptorsKhr = 1,
}

impl VkRawType<VkDescriptorUpdateTemplateType> for RawVkDescriptorUpdateTemplateType {
    fn vk_to_wrapped(src: &RawVkDescriptorUpdateTemplateType) -> VkDescriptorUpdateTemplateType {
        unsafe {
            *((src as *const i32) as *const VkDescriptorUpdateTemplateType)
        }
    }
}

impl VkWrappedType<RawVkDescriptorUpdateTemplateType> for VkDescriptorUpdateTemplateType {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplateType, dst: &mut RawVkDescriptorUpdateTemplateType) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE : VkDescriptorUpdateTemplateType = VkDescriptorUpdateTemplateType::DescriptorSet;

impl VkDefault for VkDescriptorUpdateTemplateType {
    fn vk_default() -> VkDescriptorUpdateTemplateType {
        STATIC_VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE
    }
}