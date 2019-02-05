// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkQueueFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueueFlagBits.html).
///
/// Use the macro `VkQueueFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkQueueFlags!(graphics, compute)
/// ```
/// ```
/// VkQueueFlags {
///     graphics: true,
///     compute: true,
///     ..VkQueueFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFlags {
    pub graphics: bool,
    pub compute: bool,
    pub transfer: bool,
    pub sparse_binding: bool,
    pub protected: bool,
}

#[doc(hidden)]
pub type RawVkQueueFlags = u32;

impl VkWrappedType<RawVkQueueFlags> for VkQueueFlags {
    fn vk_to_raw(src: &VkQueueFlags, dst: &mut RawVkQueueFlags) {
        *dst = 0;
        if src.graphics { *dst |= 0x00000001; }
        if src.compute { *dst |= 0x00000002; }
        if src.transfer { *dst |= 0x00000004; }
        if src.sparse_binding { *dst |= 0x00000008; }
        if src.protected { *dst |= 0x00000010; }
    }
}

impl VkRawType<VkQueueFlags> for RawVkQueueFlags {
    fn vk_to_wrapped(src: &RawVkQueueFlags) -> VkQueueFlags {
        VkQueueFlags {
            graphics: (src & 0x00000001) != 0,
            compute: (src & 0x00000002) != 0,
            transfer: (src & 0x00000004) != 0,
            sparse_binding: (src & 0x00000008) != 0,
            protected: (src & 0x00000010) != 0,
        }
    }
}

impl Default for VkQueueFlags {
    fn default() -> VkQueueFlags {
        VkQueueFlags {
            graphics: false,
            compute: false,
            transfer: false,
            sparse_binding: false,
            protected: false,
        }
    }
}

impl VkQueueFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkQueueFlags {
            graphics: false,
            compute: false,
            transfer: false,
            sparse_binding: false,
            protected: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkQueueFlags {
            graphics: true,
            compute: true,
            transfer: true,
            sparse_binding: true,
            protected: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.graphics { 0x00000001 } else { 0 }
        + if self.compute { 0x00000002 } else { 0 }
        + if self.transfer { 0x00000004 } else { 0 }
        + if self.sparse_binding { 0x00000008 } else { 0 }
        + if self.protected { 0x00000010 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkQueueFlags {
            graphics: value & 0x00000001 > 0,
            compute: value & 0x00000002 > 0,
            transfer: value & 0x00000004 > 0,
            sparse_binding: value & 0x00000008 > 0,
            protected: value & 0x00000010 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkQueueFlags {
    ( $( $x:ident ),* ) => {
        VkQueueFlags {
            $($x: true,)*
            ..VkQueueFlags::none()
        }
    }
}