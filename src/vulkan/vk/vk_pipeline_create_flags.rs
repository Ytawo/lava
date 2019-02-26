// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineCreateFlags.html).
///
/// Use the macro `VkPipelineCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineCreateFlags!(disable_optimization, allow_derivatives)
/// ```
/// ```
/// VkPipelineCreateFlags {
///     disable_optimization: true,
///     allow_derivatives: true,
///     ..VkPipelineCreateFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkPipelineCreateFlags {
    pub disable_optimization: bool,
    pub allow_derivatives: bool,
    pub derivative: bool,
    pub view_index_from_device_index: bool,
    pub dispatch_base: bool,
    pub defer_compile_nv: bool,
}

#[doc(hidden)]
pub type RawVkPipelineCreateFlags = u32;

impl VkWrappedType<RawVkPipelineCreateFlags> for VkPipelineCreateFlags {
    fn vk_to_raw(src: &VkPipelineCreateFlags, dst: &mut RawVkPipelineCreateFlags) {
        *dst = 0;
        if src.disable_optimization { *dst |= 0x00000001; }
        if src.allow_derivatives { *dst |= 0x00000002; }
        if src.derivative { *dst |= 0x00000004; }
        if src.view_index_from_device_index { *dst |= 0x00000008; }
        if src.dispatch_base { *dst |= 0x00000010; }
        if src.defer_compile_nv { *dst |= 0x00000020; }
    }
}

impl VkRawType<VkPipelineCreateFlags> for RawVkPipelineCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCreateFlags) -> VkPipelineCreateFlags {
        VkPipelineCreateFlags {
            disable_optimization: (src & 0x00000001) != 0,
            allow_derivatives: (src & 0x00000002) != 0,
            derivative: (src & 0x00000004) != 0,
            view_index_from_device_index: (src & 0x00000008) != 0,
            dispatch_base: (src & 0x00000010) != 0,
            defer_compile_nv: (src & 0x00000020) != 0,
        }
    }
}

impl Default for VkPipelineCreateFlags {
    fn default() -> VkPipelineCreateFlags {
        VkPipelineCreateFlags {
            disable_optimization: false,
            allow_derivatives: false,
            derivative: false,
            view_index_from_device_index: false,
            dispatch_base: false,
            defer_compile_nv: false,
        }
    }
}

impl VkPipelineCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineCreateFlags {
            disable_optimization: false,
            allow_derivatives: false,
            derivative: false,
            view_index_from_device_index: false,
            dispatch_base: false,
            defer_compile_nv: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineCreateFlags {
            disable_optimization: true,
            allow_derivatives: true,
            derivative: true,
            view_index_from_device_index: true,
            dispatch_base: true,
            defer_compile_nv: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.disable_optimization { 0x00000001 } else { 0 }
        + if self.allow_derivatives { 0x00000002 } else { 0 }
        + if self.derivative { 0x00000004 } else { 0 }
        + if self.view_index_from_device_index { 0x00000008 } else { 0 }
        + if self.dispatch_base { 0x00000010 } else { 0 }
        + if self.defer_compile_nv { 0x00000020 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineCreateFlags {
            disable_optimization: value & 0x00000001 > 0,
            allow_derivatives: value & 0x00000002 > 0,
            derivative: value & 0x00000004 > 0,
            view_index_from_device_index: value & 0x00000008 > 0,
            dispatch_base: value & 0x00000010 > 0,
            defer_compile_nv: value & 0x00000020 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineCreateFlags {
            $($x: true,)*
            ..VkPipelineCreateFlags::none()
        }
    }
}