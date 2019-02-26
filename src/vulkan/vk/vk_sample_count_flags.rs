// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSampleCountFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSampleCountFlags.html).
///
/// Use the macro `VkSampleCountFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkSampleCountFlags!(_1, _2)
/// ```
/// ```
/// VkSampleCountFlags {
///     _1: true,
///     _2: true,
///     ..VkSampleCountFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkSampleCountFlags {
    pub _1: bool,
    pub _2: bool,
    pub _4: bool,
    pub _8: bool,
    pub _16: bool,
    pub _32: bool,
    pub _64: bool,
}

#[doc(hidden)]
pub type RawVkSampleCountFlags = u32;

impl VkWrappedType<RawVkSampleCountFlags> for VkSampleCountFlags {
    fn vk_to_raw(src: &VkSampleCountFlags, dst: &mut RawVkSampleCountFlags) {
        *dst = 0;
        if src._1 { *dst |= 0x00000001; }
        if src._2 { *dst |= 0x00000002; }
        if src._4 { *dst |= 0x00000004; }
        if src._8 { *dst |= 0x00000008; }
        if src._16 { *dst |= 0x00000010; }
        if src._32 { *dst |= 0x00000020; }
        if src._64 { *dst |= 0x00000040; }
    }
}

impl VkRawType<VkSampleCountFlags> for RawVkSampleCountFlags {
    fn vk_to_wrapped(src: &RawVkSampleCountFlags) -> VkSampleCountFlags {
        VkSampleCountFlags {
            _1: (src & 0x00000001) != 0,
            _2: (src & 0x00000002) != 0,
            _4: (src & 0x00000004) != 0,
            _8: (src & 0x00000008) != 0,
            _16: (src & 0x00000010) != 0,
            _32: (src & 0x00000020) != 0,
            _64: (src & 0x00000040) != 0,
        }
    }
}

impl Default for VkSampleCountFlags {
    fn default() -> VkSampleCountFlags {
        VkSampleCountFlags {
            _1: false,
            _2: false,
            _4: false,
            _8: false,
            _16: false,
            _32: false,
            _64: false,
        }
    }
}

impl VkSampleCountFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkSampleCountFlags {
            _1: false,
            _2: false,
            _4: false,
            _8: false,
            _16: false,
            _32: false,
            _64: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkSampleCountFlags {
            _1: true,
            _2: true,
            _4: true,
            _8: true,
            _16: true,
            _32: true,
            _64: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self._1 { 0x00000001 } else { 0 }
        + if self._2 { 0x00000002 } else { 0 }
        + if self._4 { 0x00000004 } else { 0 }
        + if self._8 { 0x00000008 } else { 0 }
        + if self._16 { 0x00000010 } else { 0 }
        + if self._32 { 0x00000020 } else { 0 }
        + if self._64 { 0x00000040 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkSampleCountFlags {
            _1: value & 0x00000001 > 0,
            _2: value & 0x00000002 > 0,
            _4: value & 0x00000004 > 0,
            _8: value & 0x00000008 > 0,
            _16: value & 0x00000010 > 0,
            _32: value & 0x00000020 > 0,
            _64: value & 0x00000040 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkSampleCountFlags {
    ( $( $x:ident ),* ) => {
        VkSampleCountFlags {
            $($x: true,)*
            ..VkSampleCountFlags::none()
        }
    }
}