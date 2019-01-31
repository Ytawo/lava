// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryFeatureFlags {
    pub dedicated_only: bool,
    pub exportable: bool,
    pub importable: bool,
}

pub type RawVkExternalMemoryFeatureFlags = u32;

impl VkWrappedType<RawVkExternalMemoryFeatureFlags> for VkExternalMemoryFeatureFlags {
    fn vk_to_raw(src: &VkExternalMemoryFeatureFlags, dst: &mut RawVkExternalMemoryFeatureFlags) {
        *dst = 0;
        if src.dedicated_only { *dst |= 0x00000001; }
        if src.exportable { *dst |= 0x00000002; }
        if src.importable { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkExternalMemoryFeatureFlags> for RawVkExternalMemoryFeatureFlags {
    fn vk_to_wrapped(src: &RawVkExternalMemoryFeatureFlags) -> VkExternalMemoryFeatureFlags {
        VkExternalMemoryFeatureFlags {
            dedicated_only: (src & 0x00000001) != 0,
            exportable: (src & 0x00000002) != 0,
            importable: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkExternalMemoryFeatureFlags {
    fn default() -> VkExternalMemoryFeatureFlags {
        VkExternalMemoryFeatureFlags {
            dedicated_only: false,
            exportable: false,
            importable: false,
        }
    }
}

impl VkExternalMemoryFeatureFlags {
    
    pub fn none() -> VkExternalMemoryFeatureFlags {
        VkExternalMemoryFeatureFlags {
            dedicated_only: false,
            exportable: false,
            importable: false,
        }
    }
    
    pub fn all() -> VkExternalMemoryFeatureFlags {
        VkExternalMemoryFeatureFlags {
            dedicated_only: true,
            exportable: true,
            importable: true,
        }
    }
}

#[macro_export]
macro_rules! VkExternalMemoryFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkExternalMemoryFeatureFlags {
            $($x: true,)*
            ..VkExternalMemoryFeatureFlags::none()
        }
    }
}

impl VkExternalMemoryFeatureFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.dedicated_only { 0x00000001 } else { 0 }
        + if self.exportable { 0x00000002 } else { 0 }
        + if self.importable { 0x00000004 } else { 0 }
    }
}