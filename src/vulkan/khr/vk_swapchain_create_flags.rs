// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSwapchainCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html).
///
/// Use the macro `VkSwapchainCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkSwapchainCreateFlags!(split_instance_bind_regions, protected)
/// ```
/// ```
/// VkSwapchainCreateFlags {
///     split_instance_bind_regions: true,
///     protected: true,
///     ..VkSwapchainCreateFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkSwapchainCreateFlags {
    pub split_instance_bind_regions: bool,
    pub protected: bool,
    pub mutable_format: bool,
}

#[doc(hidden)]
pub type RawVkSwapchainCreateFlags = u32;

impl VkWrappedType<RawVkSwapchainCreateFlags> for VkSwapchainCreateFlags {
    fn vk_to_raw(src: &VkSwapchainCreateFlags, dst: &mut RawVkSwapchainCreateFlags) {
        *dst = 0;
        if src.split_instance_bind_regions { *dst |= 0x00000001; }
        if src.protected { *dst |= 0x00000002; }
        if src.mutable_format { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkSwapchainCreateFlags> for RawVkSwapchainCreateFlags {
    fn vk_to_wrapped(src: &RawVkSwapchainCreateFlags) -> VkSwapchainCreateFlags {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: (src & 0x00000001) != 0,
            protected: (src & 0x00000002) != 0,
            mutable_format: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkSwapchainCreateFlags {
    fn default() -> VkSwapchainCreateFlags {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: false,
            protected: false,
            mutable_format: false,
        }
    }
}

impl VkSwapchainCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: false,
            protected: false,
            mutable_format: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: true,
            protected: true,
            mutable_format: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.split_instance_bind_regions { 0x00000001 } else { 0 }
        + if self.protected { 0x00000002 } else { 0 }
        + if self.mutable_format { 0x00000004 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: value & 0x00000001 > 0,
            protected: value & 0x00000002 > 0,
            mutable_format: value & 0x00000004 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkSwapchainCreateFlags {
    ( $( $x:ident ),* ) => {
        VkSwapchainCreateFlags {
            $($x: true,)*
            ..VkSwapchainCreateFlags::none()
        }
    }
}