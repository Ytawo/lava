// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkDeviceGroupPresentModeFlags {
    pub local: bool,
    pub remote: bool,
    pub sum: bool,
    pub local_multi_device: bool,
}

pub type RawVkDeviceGroupPresentModeFlags = u32;

impl VkWrappedType<RawVkDeviceGroupPresentModeFlags> for VkDeviceGroupPresentModeFlags {
    fn vk_to_raw(src: &VkDeviceGroupPresentModeFlags, dst: &mut RawVkDeviceGroupPresentModeFlags) {
        *dst = 0;
        if src.local { *dst |= 0x00000001; }
        if src.remote { *dst |= 0x00000002; }
        if src.sum { *dst |= 0x00000004; }
        if src.local_multi_device { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkDeviceGroupPresentModeFlags> for RawVkDeviceGroupPresentModeFlags {
    fn vk_to_wrapped(src: &RawVkDeviceGroupPresentModeFlags) -> VkDeviceGroupPresentModeFlags {
        VkDeviceGroupPresentModeFlags {
            local: (src & 0x00000001) != 0,
            remote: (src & 0x00000002) != 0,
            sum: (src & 0x00000004) != 0,
            local_multi_device: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkDeviceGroupPresentModeFlags {
    fn default() -> VkDeviceGroupPresentModeFlags {
        VkDeviceGroupPresentModeFlags {
            local: false,
            remote: false,
            sum: false,
            local_multi_device: false,
        }
    }
}

impl VkDeviceGroupPresentModeFlags {
    
    pub fn none() -> VkDeviceGroupPresentModeFlags {
        VkDeviceGroupPresentModeFlags {
            local: false,
            remote: false,
            sum: false,
            local_multi_device: false,
        }
    }
    
    pub fn all() -> VkDeviceGroupPresentModeFlags {
        VkDeviceGroupPresentModeFlags {
            local: true,
            remote: true,
            sum: true,
            local_multi_device: true,
        }
    }
}

impl VkDeviceGroupPresentModeFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.local { 0x00000001 } else { 0 }
        + if self.remote { 0x00000002 } else { 0 }
        + if self.sum { 0x00000004 } else { 0 }
        + if self.local_multi_device { 0x00000008 } else { 0 }
    }
}