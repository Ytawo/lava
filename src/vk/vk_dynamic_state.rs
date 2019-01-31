// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDynamicState {
    Viewport = 0,
    Scissor = 1,
    LineWidth = 2,
    DepthBias = 3,
    BlendConstants = 4,
    DepthBounds = 5,
    StencilCompareMask = 6,
    StencilWriteMask = 7,
    StencilReference = 8,
    ViewportWScalingNv = 1000087000,
    DiscardRectangleExt = 1000099000,
    SampleLocationsExt = 1000143000,
    ViewportShadingRatePaletteNv = 1000164004,
    ViewportCoarseSampleOrderNv = 1000164006,
    ExclusiveScissorNv = 1000205001,
}

pub type RawVkDynamicState = i32;

impl VkWrappedType<RawVkDynamicState> for VkDynamicState {
    fn vk_to_raw(src: &VkDynamicState, dst: &mut RawVkDynamicState) {
        *dst = *src as i32
    }
}

impl VkRawType<VkDynamicState> for RawVkDynamicState {
    fn vk_to_wrapped(src: &RawVkDynamicState) -> VkDynamicState {
        unsafe {
            *((src as *const i32) as *const VkDynamicState)
        }
    }
}

impl Default for VkDynamicState {
    fn default() -> VkDynamicState {
        VkDynamicState::Viewport
    }
}