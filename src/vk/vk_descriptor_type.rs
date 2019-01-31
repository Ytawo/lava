// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDescriptorType {
    Sampler = 0,
    CombinedImageSampler = 1,
    SampledImage = 2,
    StorageImage = 3,
    UniformTexelBuffer = 4,
    StorageTexelBuffer = 5,
    UniformBuffer = 6,
    StorageBuffer = 7,
    UniformBufferDynamic = 8,
    StorageBufferDynamic = 9,
    InputAttachment = 10,
    InlineUniformBlockExt = 1000138000,
    AccelerationStructureNv = 1000165000,
}

pub type RawVkDescriptorType = i32;

impl VkWrappedType<RawVkDescriptorType> for VkDescriptorType {
    fn vk_to_raw(src: &VkDescriptorType, dst: &mut RawVkDescriptorType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkDescriptorType> for RawVkDescriptorType {
    fn vk_to_wrapped(src: &RawVkDescriptorType) -> VkDescriptorType {
        unsafe {
            *((src as *const i32) as *const VkDescriptorType)
        }
    }
}

impl Default for VkDescriptorType {
    fn default() -> VkDescriptorType {
        VkDescriptorType::Sampler
    }
}