// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolCreateFlags {
    pub transient: bool,
    pub reset_command_buffer: bool,
    pub protected: bool,
}

pub type RawVkCommandPoolCreateFlags = u32;

impl VkWrappedType<RawVkCommandPoolCreateFlags> for VkCommandPoolCreateFlags {
    fn vk_to_raw(src: &VkCommandPoolCreateFlags, dst: &mut RawVkCommandPoolCreateFlags) {
        *dst = 0;
        if src.transient { *dst |= 0x00000001; }
        if src.reset_command_buffer { *dst |= 0x00000002; }
        if src.protected { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkCommandPoolCreateFlags> for RawVkCommandPoolCreateFlags {
    fn vk_to_wrapped(src: &RawVkCommandPoolCreateFlags) -> VkCommandPoolCreateFlags {
        VkCommandPoolCreateFlags {
            transient: (src & 0x00000001) != 0,
            reset_command_buffer: (src & 0x00000002) != 0,
            protected: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkCommandPoolCreateFlags {
    fn default() -> VkCommandPoolCreateFlags {
        VkCommandPoolCreateFlags {
            transient: false,
            reset_command_buffer: false,
            protected: false,
        }
    }
}

impl VkCommandPoolCreateFlags {
    
    pub fn none() -> VkCommandPoolCreateFlags {
        VkCommandPoolCreateFlags {
            transient: false,
            reset_command_buffer: false,
            protected: false,
        }
    }
    
    pub fn all() -> VkCommandPoolCreateFlags {
        VkCommandPoolCreateFlags {
            transient: true,
            reset_command_buffer: true,
            protected: true,
        }
    }
}

impl VkCommandPoolCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.transient { 0x00000001 } else { 0 }
        + if self.reset_command_buffer { 0x00000002 } else { 0 }
        + if self.protected { 0x00000004 } else { 0 }
    }
}