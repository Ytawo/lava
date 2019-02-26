// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkBufferCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferCreateFlags.html).
///
/// Use the macro `VkBufferCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkBufferCreateFlags!(sparse_binding, sparse_residency)
/// ```
/// ```
/// VkBufferCreateFlags {
///     sparse_binding: true,
///     sparse_residency: true,
///     ..VkBufferCreateFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkBufferCreateFlags {
    pub sparse_binding: bool,
    pub sparse_residency: bool,
    pub sparse_aliased: bool,
    pub protected: bool,
    pub device_address_capture_replay_ext: bool,
}

#[doc(hidden)]
pub type RawVkBufferCreateFlags = u32;

impl VkWrappedType<RawVkBufferCreateFlags> for VkBufferCreateFlags {
    fn vk_to_raw(src: &VkBufferCreateFlags, dst: &mut RawVkBufferCreateFlags) {
        *dst = 0;
        if src.sparse_binding { *dst |= 0x00000001; }
        if src.sparse_residency { *dst |= 0x00000002; }
        if src.sparse_aliased { *dst |= 0x00000004; }
        if src.protected { *dst |= 0x00000008; }
        if src.device_address_capture_replay_ext { *dst |= 0x00000010; }
    }
}

impl VkRawType<VkBufferCreateFlags> for RawVkBufferCreateFlags {
    fn vk_to_wrapped(src: &RawVkBufferCreateFlags) -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: (src & 0x00000001) != 0,
            sparse_residency: (src & 0x00000002) != 0,
            sparse_aliased: (src & 0x00000004) != 0,
            protected: (src & 0x00000008) != 0,
            device_address_capture_replay_ext: (src & 0x00000010) != 0,
        }
    }
}

impl Default for VkBufferCreateFlags {
    fn default() -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: false,
            sparse_residency: false,
            sparse_aliased: false,
            protected: false,
            device_address_capture_replay_ext: false,
        }
    }
}

impl VkBufferCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkBufferCreateFlags {
            sparse_binding: false,
            sparse_residency: false,
            sparse_aliased: false,
            protected: false,
            device_address_capture_replay_ext: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkBufferCreateFlags {
            sparse_binding: true,
            sparse_residency: true,
            sparse_aliased: true,
            protected: true,
            device_address_capture_replay_ext: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.sparse_binding { 0x00000001 } else { 0 }
        + if self.sparse_residency { 0x00000002 } else { 0 }
        + if self.sparse_aliased { 0x00000004 } else { 0 }
        + if self.protected { 0x00000008 } else { 0 }
        + if self.device_address_capture_replay_ext { 0x00000010 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkBufferCreateFlags {
            sparse_binding: value & 0x00000001 > 0,
            sparse_residency: value & 0x00000002 > 0,
            sparse_aliased: value & 0x00000004 > 0,
            protected: value & 0x00000008 > 0,
            device_address_capture_replay_ext: value & 0x00000010 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkBufferCreateFlags {
    ( $( $x:ident ),* ) => {
        VkBufferCreateFlags {
            $($x: true,)*
            ..VkBufferCreateFlags::none()
        }
    }
}