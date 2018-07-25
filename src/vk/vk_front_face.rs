// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkFrontFace = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkFrontFace {
    CounterClockwise = 0,
    Clockwise = 1,
}

impl VkType<RawVkFrontFace> for VkFrontFace {
    
    fn vk_to_raw(src: &VkFrontFace, dst: &mut RawVkFrontFace) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkFrontFace) -> VkFrontFace {
        unsafe {
            *((src as *const i32) as *const VkFrontFace)
        }
    }
    
    fn vk_default() -> VkFrontFace {
        VkFrontFace::CounterClockwise
    }
}