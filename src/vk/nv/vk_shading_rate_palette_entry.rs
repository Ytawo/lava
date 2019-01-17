// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkShadingRatePaletteEntry {
    NoInvocations = 0,
    _16InvocationsPerPixel = 1,
    _8InvocationsPerPixel = 2,
    _4InvocationsPerPixel = 3,
    _2InvocationsPerPixel = 4,
    _1InvocationPerPixel = 5,
    _1InvocationPer2X1Pixels = 6,
    _1InvocationPer1X2Pixels = 7,
    _1InvocationPer2X2Pixels = 8,
    _1InvocationPer4X2Pixels = 9,
    _1InvocationPer2X4Pixels = 10,
    _1InvocationPer4X4Pixels = 11,
}

pub type RawVkShadingRatePaletteEntry = i32;

impl VkWrappedType<RawVkShadingRatePaletteEntry> for VkShadingRatePaletteEntry {
    fn vk_to_raw(src: &VkShadingRatePaletteEntry, dst: &mut RawVkShadingRatePaletteEntry) {
        *dst = *src as i32
    }
}

impl VkRawType<VkShadingRatePaletteEntry> for RawVkShadingRatePaletteEntry {
    fn vk_to_wrapped(src: &RawVkShadingRatePaletteEntry) -> VkShadingRatePaletteEntry {
        unsafe {
            *((src as *const i32) as *const VkShadingRatePaletteEntry)
        }
    }
}

impl Default for VkShadingRatePaletteEntry {
    fn default() -> VkShadingRatePaletteEntry {
        VkShadingRatePaletteEntry::NoInvocations
    }
}