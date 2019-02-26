// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineRasterizationConservativeStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html).
///
/// Use the macro `VkPipelineRasterizationConservativeStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineRasterizationConservativeStateCreateFlags!()
/// ```
/// ```
/// VkPipelineRasterizationConservativeStateCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationConservativeStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineRasterizationConservativeStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineRasterizationConservativeStateCreateFlags> for VkPipelineRasterizationConservativeStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineRasterizationConservativeStateCreateFlags, dst: &mut RawVkPipelineRasterizationConservativeStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineRasterizationConservativeStateCreateFlags> for RawVkPipelineRasterizationConservativeStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationConservativeStateCreateFlags) -> VkPipelineRasterizationConservativeStateCreateFlags {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineRasterizationConservativeStateCreateFlags {
    fn default() -> VkPipelineRasterizationConservativeStateCreateFlags {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
}

impl VkPipelineRasterizationConservativeStateCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineRasterizationConservativeStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineRasterizationConservativeStateCreateFlags {
            $($x: true,)*
            ..VkPipelineRasterizationConservativeStateCreateFlags::none()
        }
    }
}