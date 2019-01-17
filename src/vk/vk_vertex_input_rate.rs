// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkVertexInputRate {
    Vertex = 0,
    Instance = 1,
}

pub type RawVkVertexInputRate = i32;

impl VkWrappedType<RawVkVertexInputRate> for VkVertexInputRate {
    fn vk_to_raw(src: &VkVertexInputRate, dst: &mut RawVkVertexInputRate) {
        *dst = *src as i32
    }
}

impl VkRawType<VkVertexInputRate> for RawVkVertexInputRate {
    fn vk_to_wrapped(src: &RawVkVertexInputRate) -> VkVertexInputRate {
        unsafe {
            *((src as *const i32) as *const VkVertexInputRate)
        }
    }
}

impl Default for VkVertexInputRate {
    fn default() -> VkVertexInputRate {
        VkVertexInputRate::Vertex
    }
}