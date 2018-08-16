// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkDisplayPowerState = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDisplayPowerState {
    Off = 0,
    Suspend = 1,
    On = 2,
}

impl VkRawType<VkDisplayPowerState> for RawVkDisplayPowerState {
    fn vk_to_wrapped(src: &RawVkDisplayPowerState) -> VkDisplayPowerState {
        unsafe {
            *((src as *const i32) as *const VkDisplayPowerState)
        }
    }
}

impl VkWrappedType<RawVkDisplayPowerState> for VkDisplayPowerState {
    fn vk_to_raw(src: &VkDisplayPowerState, dst: &mut RawVkDisplayPowerState) {
        *dst = *src as i32
    }
}

impl Default for VkDisplayPowerState {
    fn default() -> VkDisplayPowerState {
        VkDisplayPowerState::Off
    }
}