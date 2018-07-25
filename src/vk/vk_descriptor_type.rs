// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkDescriptorType = i32;

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
}

impl VkType<RawVkDescriptorType> for VkDescriptorType {
    
    fn vk_to_raw(src: &VkDescriptorType, dst: &mut RawVkDescriptorType) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkDescriptorType) -> VkDescriptorType {
        unsafe {
            *((src as *const i32) as *const VkDescriptorType)
        }
    }
    
    fn vk_default() -> VkDescriptorType {
        VkDescriptorType::Sampler
    }
}