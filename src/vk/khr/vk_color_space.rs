// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkColorSpace {
    SrgbNonlinear = 0,
    DisplayP3NonlinearExt = 1000104001,
    ExtendedSrgbLinearExt = 1000104002,
    DciP3LinearExt = 1000104003,
    DciP3NonlinearExt = 1000104004,
    Bt709LinearExt = 1000104005,
    Bt709NonlinearExt = 1000104006,
    Bt2020LinearExt = 1000104007,
    Hdr10St2084Ext = 1000104008,
    DolbyvisionExt = 1000104009,
    Hdr10HlgExt = 1000104010,
    AdobergbLinearExt = 1000104011,
    AdobergbNonlinearExt = 1000104012,
    PassThroughExt = 1000104013,
    ExtendedSrgbNonlinearExt = 1000104014,
}

pub type RawVkColorSpace = i32;

impl VkWrappedType<RawVkColorSpace> for VkColorSpace {
    fn vk_to_raw(src: &VkColorSpace, dst: &mut RawVkColorSpace) {
        *dst = *src as i32
    }
}

impl VkRawType<VkColorSpace> for RawVkColorSpace {
    fn vk_to_wrapped(src: &RawVkColorSpace) -> VkColorSpace {
        unsafe {
            *((src as *const i32) as *const VkColorSpace)
        }
    }
}

impl Default for VkColorSpace {
    fn default() -> VkColorSpace {
        VkColorSpace::SrgbNonlinear
    }
}