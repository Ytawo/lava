// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkChromaLocation {
    CositedEven = 0,
    Midpoint = 1,
}

pub type RawVkChromaLocation = i32;

impl VkWrappedType<RawVkChromaLocation> for VkChromaLocation {
    fn vk_to_raw(src: &VkChromaLocation, dst: &mut RawVkChromaLocation) {
        *dst = *src as i32
    }
}

impl VkRawType<VkChromaLocation> for RawVkChromaLocation {
    fn vk_to_wrapped(src: &RawVkChromaLocation) -> VkChromaLocation {
        unsafe {
            *((src as *const i32) as *const VkChromaLocation)
        }
    }
}

impl Default for VkChromaLocation {
    fn default() -> VkChromaLocation {
        VkChromaLocation::CositedEven
    }
}