// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCommandBufferUsageFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferUsageFlags.html).
///
/// Use the macro `VkCommandBufferUsageFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkCommandBufferUsageFlags!(one_time_submit, render_pass_continue)
/// ```
/// ```
/// VkCommandBufferUsageFlags {
///     one_time_submit: true,
///     render_pass_continue: true,
///     ..VkCommandBufferUsageFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkCommandBufferUsageFlags {
    pub one_time_submit: bool,
    pub render_pass_continue: bool,
    pub simultaneous_use: bool,
}

#[doc(hidden)]
pub type RawVkCommandBufferUsageFlags = u32;

impl VkWrappedType<RawVkCommandBufferUsageFlags> for VkCommandBufferUsageFlags {
    fn vk_to_raw(src: &VkCommandBufferUsageFlags, dst: &mut RawVkCommandBufferUsageFlags) {
        *dst = 0;
        if src.one_time_submit { *dst |= 0x00000001; }
        if src.render_pass_continue { *dst |= 0x00000002; }
        if src.simultaneous_use { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkCommandBufferUsageFlags> for RawVkCommandBufferUsageFlags {
    fn vk_to_wrapped(src: &RawVkCommandBufferUsageFlags) -> VkCommandBufferUsageFlags {
        VkCommandBufferUsageFlags {
            one_time_submit: (src & 0x00000001) != 0,
            render_pass_continue: (src & 0x00000002) != 0,
            simultaneous_use: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkCommandBufferUsageFlags {
    fn default() -> VkCommandBufferUsageFlags {
        VkCommandBufferUsageFlags {
            one_time_submit: false,
            render_pass_continue: false,
            simultaneous_use: false,
        }
    }
}

impl VkCommandBufferUsageFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkCommandBufferUsageFlags {
            one_time_submit: false,
            render_pass_continue: false,
            simultaneous_use: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkCommandBufferUsageFlags {
            one_time_submit: true,
            render_pass_continue: true,
            simultaneous_use: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.one_time_submit { 0x00000001 } else { 0 }
        + if self.render_pass_continue { 0x00000002 } else { 0 }
        + if self.simultaneous_use { 0x00000004 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkCommandBufferUsageFlags {
            one_time_submit: value & 0x00000001 > 0,
            render_pass_continue: value & 0x00000002 > 0,
            simultaneous_use: value & 0x00000004 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkCommandBufferUsageFlags {
    ( $( $x:ident ),* ) => {
        VkCommandBufferUsageFlags {
            $($x: true,)*
            ..VkCommandBufferUsageFlags::none()
        }
    }
}