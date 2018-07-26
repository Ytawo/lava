// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkMemoryPropertyFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkMemoryPropertyFlags {
    device_local: bool,
    host_visible: bool,
    host_coherent: bool,
    host_cached: bool,
    lazily_allocated: bool,
    protected: bool,
}

impl VkRawType<VkMemoryPropertyFlags> for RawVkMemoryPropertyFlags {
    
    fn vk_to_wrapped(src: &RawVkMemoryPropertyFlags) -> VkMemoryPropertyFlags {
        VkMemoryPropertyFlags {
            device_local: (src & 0x00000001) != 0,
            host_visible: (src & 0x00000002) != 0,
            host_coherent: (src & 0x00000004) != 0,
            host_cached: (src & 0x00000008) != 0,
            lazily_allocated: (src & 0x00000010) != 0,
            protected: (src & 0x00000020) != 0,
        }
    }
}

impl VkWrappedType<RawVkMemoryPropertyFlags> for VkMemoryPropertyFlags {
    
    fn vk_to_raw(src: &VkMemoryPropertyFlags, dst: &mut RawVkMemoryPropertyFlags) {
        *dst = 0;
        if src.device_local { *dst |= 0x00000001; }
        if src.host_visible { *dst |= 0x00000002; }
        if src.host_coherent { *dst |= 0x00000004; }
        if src.host_cached { *dst |= 0x00000008; }
        if src.lazily_allocated { *dst |= 0x00000010; }
        if src.protected { *dst |= 0x00000020; }
    }
}

impl VkDefault for VkMemoryPropertyFlags {
    
    fn vk_default() -> VkMemoryPropertyFlags {
        VkMemoryPropertyFlags {
            device_local: false,
            host_visible: false,
            host_coherent: false,
            host_cached: false,
            lazily_allocated: false,
            protected: false,
        }
    }
}