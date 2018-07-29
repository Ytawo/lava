// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPolygonMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
    FillRectangleNv = 1000153000,
}

impl VkRawType<VkPolygonMode> for RawVkPolygonMode {
    fn vk_to_wrapped(src: &RawVkPolygonMode) -> VkPolygonMode {
        unsafe {
            *((src as *const i32) as *const VkPolygonMode)
        }
    }
}

impl VkWrappedType<RawVkPolygonMode> for VkPolygonMode {
    fn vk_to_raw(src: &VkPolygonMode, dst: &mut RawVkPolygonMode) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_POLYGON_MODE : VkPolygonMode = VkPolygonMode::Fill;

impl VkDefault for VkPolygonMode {
    fn vk_default() -> VkPolygonMode {
        STATIC_VK_POLYGON_MODE
    }
}