// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkFramebufferCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFramebufferCreateFlags.html).
///
/// Use the macro `VkFramebufferCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkFramebufferCreateFlags!()
/// ```
/// ```
/// VkFramebufferCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkFramebufferCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkFramebufferCreateFlags = u32;

impl VkWrappedType<RawVkFramebufferCreateFlags> for VkFramebufferCreateFlags {
    fn vk_to_raw(src: &VkFramebufferCreateFlags, dst: &mut RawVkFramebufferCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkFramebufferCreateFlags> for RawVkFramebufferCreateFlags {
    fn vk_to_wrapped(src: &RawVkFramebufferCreateFlags) -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
}

impl Default for VkFramebufferCreateFlags {
    fn default() -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
}

impl VkFramebufferCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkFramebufferCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkFramebufferCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkFramebufferCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkFramebufferCreateFlags {
    ( $( $x:ident ),* ) => {
        VkFramebufferCreateFlags {
            $($x: true,)*
            ..VkFramebufferCreateFlags::none()
        }
    }
}