// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDeviceDiagnosticsConfigFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagsNV.html).
///
/// Use the macro `VkDeviceDiagnosticsConfigFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDeviceDiagnosticsConfigFlags!(enable_shader_debug_info, enable_resource_tracking)
/// ```
/// ```
/// VkDeviceDiagnosticsConfigFlags {
///     enable_shader_debug_info: true,
///     enable_resource_tracking: true,
///     ..VkDeviceDiagnosticsConfigFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkDeviceDiagnosticsConfigFlags {
    pub enable_shader_debug_info: bool,
    pub enable_resource_tracking: bool,
    pub enable_automatic_checkpoints: bool,
}

#[doc(hidden)]
pub type RawVkDeviceDiagnosticsConfigFlags = u32;

impl VkWrappedType<RawVkDeviceDiagnosticsConfigFlags> for VkDeviceDiagnosticsConfigFlags {
    fn vk_to_raw(src: &VkDeviceDiagnosticsConfigFlags, dst: &mut RawVkDeviceDiagnosticsConfigFlags) {
        *dst = 0;
        if src.enable_shader_debug_info { *dst |= 0x00000001; }
        if src.enable_resource_tracking { *dst |= 0x00000002; }
        if src.enable_automatic_checkpoints { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkDeviceDiagnosticsConfigFlags> for RawVkDeviceDiagnosticsConfigFlags {
    fn vk_to_wrapped(src: &RawVkDeviceDiagnosticsConfigFlags) -> VkDeviceDiagnosticsConfigFlags {
        VkDeviceDiagnosticsConfigFlags {
            enable_shader_debug_info: (src & 0x00000001) != 0,
            enable_resource_tracking: (src & 0x00000002) != 0,
            enable_automatic_checkpoints: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkDeviceDiagnosticsConfigFlags {
    fn default() -> VkDeviceDiagnosticsConfigFlags {
        VkDeviceDiagnosticsConfigFlags {
            enable_shader_debug_info: false,
            enable_resource_tracking: false,
            enable_automatic_checkpoints: false,
        }
    }
}

impl VkDeviceDiagnosticsConfigFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDeviceDiagnosticsConfigFlags {
            enable_shader_debug_info: false,
            enable_resource_tracking: false,
            enable_automatic_checkpoints: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDeviceDiagnosticsConfigFlags {
            enable_shader_debug_info: true,
            enable_resource_tracking: true,
            enable_automatic_checkpoints: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.enable_shader_debug_info { 0x00000001 } else { 0 }
        + if self.enable_resource_tracking { 0x00000002 } else { 0 }
        + if self.enable_automatic_checkpoints { 0x00000004 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDeviceDiagnosticsConfigFlags {
            enable_shader_debug_info: value & 0x00000001 > 0,
            enable_resource_tracking: value & 0x00000002 > 0,
            enable_automatic_checkpoints: value & 0x00000004 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDeviceDiagnosticsConfigFlags {
    ( $( $x:ident ),* ) => {
        VkDeviceDiagnosticsConfigFlags {
            $($x: true,)*
            ..VkDeviceDiagnosticsConfigFlags::none()
        }
    }
}