// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkPolygonMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
    FillRectangleNv = 1000153000,
}

impl VkType<RawVkPolygonMode> for VkPolygonMode {
    
    fn vk_to_raw(src: &VkPolygonMode, dst: &mut RawVkPolygonMode) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkPolygonMode) -> VkPolygonMode {
        unsafe {
            *((src as *const i32) as *const VkPolygonMode)
        }
    }
    
    fn vk_default() -> VkPolygonMode {
        VkPolygonMode::Fill
    }
}