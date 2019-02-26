// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkInstanceCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkInstanceCreateFlags.html).
///
/// Use the macro `VkInstanceCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkInstanceCreateFlags!()
/// ```
/// ```
/// VkInstanceCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkInstanceCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkInstanceCreateFlags = u32;

impl VkWrappedType<RawVkInstanceCreateFlags> for VkInstanceCreateFlags {
    fn vk_to_raw(src: &VkInstanceCreateFlags, dst: &mut RawVkInstanceCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkInstanceCreateFlags> for RawVkInstanceCreateFlags {
    fn vk_to_wrapped(src: &RawVkInstanceCreateFlags) -> VkInstanceCreateFlags {
        VkInstanceCreateFlags {
            
        }
    }
}

impl Default for VkInstanceCreateFlags {
    fn default() -> VkInstanceCreateFlags {
        VkInstanceCreateFlags {
            
        }
    }
}

impl VkInstanceCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkInstanceCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkInstanceCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkInstanceCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkInstanceCreateFlags {
    ( $( $x:ident ),* ) => {
        VkInstanceCreateFlags {
            $($x: true,)*
            ..VkInstanceCreateFlags::none()
        }
    }
}