// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkShaderInfoType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkShaderInfoType {
    Statistics = 0,
    Binary = 1,
    Disassembly = 2,
}

impl VkType<RawVkShaderInfoType> for VkShaderInfoType {
    
    fn vk_to_raw(src: &VkShaderInfoType, dst: &mut RawVkShaderInfoType) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkShaderInfoType) -> VkShaderInfoType {
        unsafe {
            *((src as *const i32) as *const VkShaderInfoType)
        }
    }
    
    fn vk_default() -> VkShaderInfoType {
        VkShaderInfoType::Statistics
    }
}