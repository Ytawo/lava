// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineCreationFeedbackFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagsEXT.html).
///
/// Use the macro `VkPipelineCreationFeedbackFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineCreationFeedbackFlags!(valid, application_pipeline_cache_hit)
/// ```
/// ```
/// VkPipelineCreationFeedbackFlags {
///     valid: true,
///     application_pipeline_cache_hit: true,
///     ..VkPipelineCreationFeedbackFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkPipelineCreationFeedbackFlags {
    pub valid: bool,
    pub application_pipeline_cache_hit: bool,
    pub base_pipeline_acceleration: bool,
}

#[doc(hidden)]
pub type RawVkPipelineCreationFeedbackFlags = u32;

impl VkWrappedType<RawVkPipelineCreationFeedbackFlags> for VkPipelineCreationFeedbackFlags {
    fn vk_to_raw(src: &VkPipelineCreationFeedbackFlags, dst: &mut RawVkPipelineCreationFeedbackFlags) {
        *dst = 0;
        if src.valid { *dst |= 0x00000001; }
        if src.application_pipeline_cache_hit { *dst |= 0x00000002; }
        if src.base_pipeline_acceleration { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkPipelineCreationFeedbackFlags> for RawVkPipelineCreationFeedbackFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCreationFeedbackFlags) -> VkPipelineCreationFeedbackFlags {
        VkPipelineCreationFeedbackFlags {
            valid: (src & 0x00000001) != 0,
            application_pipeline_cache_hit: (src & 0x00000002) != 0,
            base_pipeline_acceleration: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkPipelineCreationFeedbackFlags {
    fn default() -> VkPipelineCreationFeedbackFlags {
        VkPipelineCreationFeedbackFlags {
            valid: false,
            application_pipeline_cache_hit: false,
            base_pipeline_acceleration: false,
        }
    }
}

impl VkPipelineCreationFeedbackFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineCreationFeedbackFlags {
            valid: false,
            application_pipeline_cache_hit: false,
            base_pipeline_acceleration: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineCreationFeedbackFlags {
            valid: true,
            application_pipeline_cache_hit: true,
            base_pipeline_acceleration: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.valid { 0x00000001 } else { 0 }
        + if self.application_pipeline_cache_hit { 0x00000002 } else { 0 }
        + if self.base_pipeline_acceleration { 0x00000004 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineCreationFeedbackFlags {
            valid: value & 0x00000001 > 0,
            application_pipeline_cache_hit: value & 0x00000002 > 0,
            base_pipeline_acceleration: value & 0x00000004 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineCreationFeedbackFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineCreationFeedbackFlags {
            $($x: true,)*
            ..VkPipelineCreationFeedbackFlags::none()
        }
    }
}