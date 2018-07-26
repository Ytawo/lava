// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDebugReportFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkDebugReportFlags {
    information: bool,
    warning: bool,
    performance_warning: bool,
    error: bool,
    debug: bool,
}

impl VkRawType<VkDebugReportFlags> for RawVkDebugReportFlags {
    
    fn vk_to_wrapped(src: &RawVkDebugReportFlags) -> VkDebugReportFlags {
        VkDebugReportFlags {
            information: (src & 0x00000001) != 0,
            warning: (src & 0x00000002) != 0,
            performance_warning: (src & 0x00000004) != 0,
            error: (src & 0x00000008) != 0,
            debug: (src & 0x00000010) != 0,
        }
    }
}

impl VkWrappedType<RawVkDebugReportFlags> for VkDebugReportFlags {
    
    fn vk_to_raw(src: &VkDebugReportFlags, dst: &mut RawVkDebugReportFlags) {
        *dst = 0;
        if src.information { *dst |= 0x00000001; }
        if src.warning { *dst |= 0x00000002; }
        if src.performance_warning { *dst |= 0x00000004; }
        if src.error { *dst |= 0x00000008; }
        if src.debug { *dst |= 0x00000010; }
    }
}

impl VkDefault for VkDebugReportFlags {
    
    fn vk_default() -> VkDebugReportFlags {
        VkDebugReportFlags {
            information: false,
            warning: false,
            performance_warning: false,
            error: false,
            debug: false,
        }
    }
}