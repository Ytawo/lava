// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkExternalFenceFeatureFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkExternalFenceFeatureFlags {
    exportable: bool,
    importable: bool,
}

impl VkRawType<VkExternalFenceFeatureFlags> for RawVkExternalFenceFeatureFlags {
    fn vk_to_wrapped(src: &RawVkExternalFenceFeatureFlags) -> VkExternalFenceFeatureFlags {
        VkExternalFenceFeatureFlags {
            exportable: (src & 0x00000001) != 0,
            importable: (src & 0x00000002) != 0,
        }
    }
}

impl VkWrappedType<RawVkExternalFenceFeatureFlags> for VkExternalFenceFeatureFlags {
    fn vk_to_raw(src: &VkExternalFenceFeatureFlags, dst: &mut RawVkExternalFenceFeatureFlags) {
        *dst = 0;
        if src.exportable { *dst |= 0x00000001; }
        if src.importable { *dst |= 0x00000002; }
    }
}

pub static STATIC_VK_EXTERNAL_FENCE_FEATURE_FLAGS : VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlags {
    exportable: false,
    importable: false,
};

impl VkDefault for VkExternalFenceFeatureFlags {
    fn vk_default() -> VkExternalFenceFeatureFlags {
        STATIC_VK_EXTERNAL_FENCE_FEATURE_FLAGS
    }
}