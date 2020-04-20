// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreateFlags.html).
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
    pub ray_tracing_no_null_any_hit_shaders_khr: bool,
    pub ray_tracing_no_null_closest_hit_shaders_khr: bool,
    pub ray_tracing_no_null_miss_shaders_khr: bool,
    pub ray_tracing_no_null_intersection_shaders_khr: bool,
    pub ray_tracing_skip_triangles_khr: bool,
    pub ray_tracing_skip_aabbs_khr: bool,
    pub defer_compile_nv: bool,
    pub capture_statistics_khr: bool,
    pub capture_internal_representations_khr: bool,
    pub indirect_bindable_nv: bool,
    pub library_khr: bool,
    pub fail_on_pipeline_compile_required_ext: bool,
    pub early_return_on_failure_ext: bool,
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
        if src.ray_tracing_no_null_any_hit_shaders_khr { *dst |= 0x00004000; }
        if src.ray_tracing_no_null_closest_hit_shaders_khr { *dst |= 0x00008000; }
        if src.ray_tracing_no_null_miss_shaders_khr { *dst |= 0x00010000; }
        if src.ray_tracing_no_null_intersection_shaders_khr { *dst |= 0x00020000; }
        if src.ray_tracing_skip_triangles_khr { *dst |= 0x00001000; }
        if src.ray_tracing_skip_aabbs_khr { *dst |= 0x00002000; }
        if src.defer_compile_nv { *dst |= 0x00000020; }
        if src.capture_statistics_khr { *dst |= 0x00000040; }
        if src.capture_internal_representations_khr { *dst |= 0x00000080; }
        if src.indirect_bindable_nv { *dst |= 0x00040000; }
        if src.library_khr { *dst |= 0x00000800; }
        if src.fail_on_pipeline_compile_required_ext { *dst |= 0x00000100; }
        if src.early_return_on_failure_ext { *dst |= 0x00000200; }
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
            ray_tracing_no_null_any_hit_shaders_khr: (src & 0x00004000) != 0,
            ray_tracing_no_null_closest_hit_shaders_khr: (src & 0x00008000) != 0,
            ray_tracing_no_null_miss_shaders_khr: (src & 0x00010000) != 0,
            ray_tracing_no_null_intersection_shaders_khr: (src & 0x00020000) != 0,
            ray_tracing_skip_triangles_khr: (src & 0x00001000) != 0,
            ray_tracing_skip_aabbs_khr: (src & 0x00002000) != 0,
            defer_compile_nv: (src & 0x00000020) != 0,
            capture_statistics_khr: (src & 0x00000040) != 0,
            capture_internal_representations_khr: (src & 0x00000080) != 0,
            indirect_bindable_nv: (src & 0x00040000) != 0,
            library_khr: (src & 0x00000800) != 0,
            fail_on_pipeline_compile_required_ext: (src & 0x00000100) != 0,
            early_return_on_failure_ext: (src & 0x00000200) != 0,
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
            ray_tracing_no_null_any_hit_shaders_khr: false,
            ray_tracing_no_null_closest_hit_shaders_khr: false,
            ray_tracing_no_null_miss_shaders_khr: false,
            ray_tracing_no_null_intersection_shaders_khr: false,
            ray_tracing_skip_triangles_khr: false,
            ray_tracing_skip_aabbs_khr: false,
            defer_compile_nv: false,
            capture_statistics_khr: false,
            capture_internal_representations_khr: false,
            indirect_bindable_nv: false,
            library_khr: false,
            fail_on_pipeline_compile_required_ext: false,
            early_return_on_failure_ext: false,
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
            ray_tracing_no_null_any_hit_shaders_khr: false,
            ray_tracing_no_null_closest_hit_shaders_khr: false,
            ray_tracing_no_null_miss_shaders_khr: false,
            ray_tracing_no_null_intersection_shaders_khr: false,
            ray_tracing_skip_triangles_khr: false,
            ray_tracing_skip_aabbs_khr: false,
            defer_compile_nv: false,
            capture_statistics_khr: false,
            capture_internal_representations_khr: false,
            indirect_bindable_nv: false,
            library_khr: false,
            fail_on_pipeline_compile_required_ext: false,
            early_return_on_failure_ext: false,
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
            ray_tracing_no_null_any_hit_shaders_khr: true,
            ray_tracing_no_null_closest_hit_shaders_khr: true,
            ray_tracing_no_null_miss_shaders_khr: true,
            ray_tracing_no_null_intersection_shaders_khr: true,
            ray_tracing_skip_triangles_khr: true,
            ray_tracing_skip_aabbs_khr: true,
            defer_compile_nv: true,
            capture_statistics_khr: true,
            capture_internal_representations_khr: true,
            indirect_bindable_nv: true,
            library_khr: true,
            fail_on_pipeline_compile_required_ext: true,
            early_return_on_failure_ext: true,
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
        + if self.ray_tracing_no_null_any_hit_shaders_khr { 0x00004000 } else { 0 }
        + if self.ray_tracing_no_null_closest_hit_shaders_khr { 0x00008000 } else { 0 }
        + if self.ray_tracing_no_null_miss_shaders_khr { 0x00010000 } else { 0 }
        + if self.ray_tracing_no_null_intersection_shaders_khr { 0x00020000 } else { 0 }
        + if self.ray_tracing_skip_triangles_khr { 0x00001000 } else { 0 }
        + if self.ray_tracing_skip_aabbs_khr { 0x00002000 } else { 0 }
        + if self.defer_compile_nv { 0x00000020 } else { 0 }
        + if self.capture_statistics_khr { 0x00000040 } else { 0 }
        + if self.capture_internal_representations_khr { 0x00000080 } else { 0 }
        + if self.indirect_bindable_nv { 0x00040000 } else { 0 }
        + if self.library_khr { 0x00000800 } else { 0 }
        + if self.fail_on_pipeline_compile_required_ext { 0x00000100 } else { 0 }
        + if self.early_return_on_failure_ext { 0x00000200 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineCreateFlags {
            disable_optimization: value & 0x00000001 > 0,
            allow_derivatives: value & 0x00000002 > 0,
            derivative: value & 0x00000004 > 0,
            view_index_from_device_index: value & 0x00000008 > 0,
            dispatch_base: value & 0x00000010 > 0,
            ray_tracing_no_null_any_hit_shaders_khr: value & 0x00004000 > 0,
            ray_tracing_no_null_closest_hit_shaders_khr: value & 0x00008000 > 0,
            ray_tracing_no_null_miss_shaders_khr: value & 0x00010000 > 0,
            ray_tracing_no_null_intersection_shaders_khr: value & 0x00020000 > 0,
            ray_tracing_skip_triangles_khr: value & 0x00001000 > 0,
            ray_tracing_skip_aabbs_khr: value & 0x00002000 > 0,
            defer_compile_nv: value & 0x00000020 > 0,
            capture_statistics_khr: value & 0x00000040 > 0,
            capture_internal_representations_khr: value & 0x00000080 > 0,
            indirect_bindable_nv: value & 0x00040000 > 0,
            library_khr: value & 0x00000800 > 0,
            fail_on_pipeline_compile_required_ext: value & 0x00000100 > 0,
            early_return_on_failure_ext: value & 0x00000200 > 0,
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