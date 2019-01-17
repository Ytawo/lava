// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkDependencyFlags {
    pub by_region: bool,
    pub device_group: bool,
    pub view_local: bool,
}

pub type RawVkDependencyFlags = u32;

impl VkWrappedType<RawVkDependencyFlags> for VkDependencyFlags {
    fn vk_to_raw(src: &VkDependencyFlags, dst: &mut RawVkDependencyFlags) {
        *dst = 0;
        if src.by_region { *dst |= 0x00000001; }
        if src.device_group { *dst |= 0x00000004; }
        if src.view_local { *dst |= 0x00000002; }
    }
}

impl VkRawType<VkDependencyFlags> for RawVkDependencyFlags {
    fn vk_to_wrapped(src: &RawVkDependencyFlags) -> VkDependencyFlags {
        VkDependencyFlags {
            by_region: (src & 0x00000001) != 0,
            device_group: (src & 0x00000004) != 0,
            view_local: (src & 0x00000002) != 0,
        }
    }
}

impl Default for VkDependencyFlags {
    fn default() -> VkDependencyFlags {
        VkDependencyFlags {
            by_region: false,
            device_group: false,
            view_local: false,
        }
    }
}

impl VkDependencyFlags {
    
    pub fn none() -> VkDependencyFlags {
        VkDependencyFlags {
            by_region: false,
            device_group: false,
            view_local: false,
        }
    }
    
    pub fn all() -> VkDependencyFlags {
        VkDependencyFlags {
            by_region: true,
            device_group: true,
            view_local: true,
        }
    }
}

impl VkDependencyFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.by_region { 0x00000001 } else { 0 }
        + if self.device_group { 0x00000004 } else { 0 }
        + if self.view_local { 0x00000002 } else { 0 }
    }
}