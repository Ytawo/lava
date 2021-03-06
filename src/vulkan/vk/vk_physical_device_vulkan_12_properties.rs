// Generated by `scripts/generate.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::vk::{VkDriverId,RawVkDriverId};
use vulkan::vk::{VkConformanceVersion,RawVkConformanceVersion};
use vulkan::vk::{VkShaderFloatControlsIndependence,RawVkShaderFloatControlsIndependence};
use vulkan::vk::{VkResolveModeFlags,RawVkResolveModeFlags};
use vulkan::vk::{VkSampleCountFlags,RawVkSampleCountFlags};

/// Wrapper for [VkPhysicalDeviceVulkan12Properties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceVulkan12Properties {
    pub driver_id: VkDriverId,
    pub driver_name: String,
    pub driver_info: String,
    pub conformance_version: VkConformanceVersion,
    pub denorm_behavior_independence: VkShaderFloatControlsIndependence,
    pub rounding_mode_independence: VkShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float_16: bool,
    pub shader_signed_zero_inf_nan_preserve_float_32: bool,
    pub shader_signed_zero_inf_nan_preserve_float_64: bool,
    pub shader_denorm_preserve_float_16: bool,
    pub shader_denorm_preserve_float_32: bool,
    pub shader_denorm_preserve_float_64: bool,
    pub shader_denorm_flush_to_zero_float_16: bool,
    pub shader_denorm_flush_to_zero_float_32: bool,
    pub shader_denorm_flush_to_zero_float_64: bool,
    pub shader_rounding_mode_rtefloat_16: bool,
    pub shader_rounding_mode_rtefloat_32: bool,
    pub shader_rounding_mode_rtefloat_64: bool,
    pub shader_rounding_mode_rtzfloat_16: bool,
    pub shader_rounding_mode_rtzfloat_32: bool,
    pub shader_rounding_mode_rtzfloat_64: bool,
    pub max_update_after_bind_descriptors_in_all_pools: usize,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: bool,
    pub shader_sampled_image_array_non_uniform_indexing_native: bool,
    pub shader_storage_buffer_array_non_uniform_indexing_native: bool,
    pub shader_storage_image_array_non_uniform_indexing_native: bool,
    pub shader_input_attachment_array_non_uniform_indexing_native: bool,
    pub robust_buffer_access_update_after_bind: bool,
    pub quad_divergent_implicit_lod: bool,
    pub max_per_stage_descriptor_update_after_bind_samplers: usize,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: usize,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: usize,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: usize,
    pub max_per_stage_descriptor_update_after_bind_storage_images: usize,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: usize,
    pub max_per_stage_update_after_bind_resources: usize,
    pub max_descriptor_set_update_after_bind_samplers: usize,
    pub max_descriptor_set_update_after_bind_uniform_buffers: usize,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: usize,
    pub max_descriptor_set_update_after_bind_storage_buffers: usize,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: usize,
    pub max_descriptor_set_update_after_bind_sampled_images: usize,
    pub max_descriptor_set_update_after_bind_storage_images: usize,
    pub max_descriptor_set_update_after_bind_input_attachments: usize,
    pub supported_depth_resolve_modes: VkResolveModeFlags,
    pub supported_stencil_resolve_modes: VkResolveModeFlags,
    pub independent_resolve_none: bool,
    pub independent_resolve: bool,
    pub filter_minmax_single_component_formats: bool,
    pub filter_minmax_image_component_mapping: bool,
    pub max_timeline_semaphore_value_difference: usize,
    pub framebuffer_integer_color_sample_counts: VkSampleCountFlags,
}

#[doc(hidden)]
#[repr(C)]
pub struct RawVkPhysicalDeviceVulkan12Properties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub driver_id: RawVkDriverId,
    pub driver_name: [c_char; 256],
    pub driver_info: [c_char; 256],
    pub conformance_version: RawVkConformanceVersion,
    pub denorm_behavior_independence: RawVkShaderFloatControlsIndependence,
    pub rounding_mode_independence: RawVkShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float_16: u32,
    pub shader_signed_zero_inf_nan_preserve_float_32: u32,
    pub shader_signed_zero_inf_nan_preserve_float_64: u32,
    pub shader_denorm_preserve_float_16: u32,
    pub shader_denorm_preserve_float_32: u32,
    pub shader_denorm_preserve_float_64: u32,
    pub shader_denorm_flush_to_zero_float_16: u32,
    pub shader_denorm_flush_to_zero_float_32: u32,
    pub shader_denorm_flush_to_zero_float_64: u32,
    pub shader_rounding_mode_rtefloat_16: u32,
    pub shader_rounding_mode_rtefloat_32: u32,
    pub shader_rounding_mode_rtefloat_64: u32,
    pub shader_rounding_mode_rtzfloat_16: u32,
    pub shader_rounding_mode_rtzfloat_32: u32,
    pub shader_rounding_mode_rtzfloat_64: u32,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: u32,
    pub shader_sampled_image_array_non_uniform_indexing_native: u32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: u32,
    pub shader_storage_image_array_non_uniform_indexing_native: u32,
    pub shader_input_attachment_array_non_uniform_indexing_native: u32,
    pub robust_buffer_access_update_after_bind: u32,
    pub quad_divergent_implicit_lod: u32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    pub supported_depth_resolve_modes: RawVkResolveModeFlags,
    pub supported_stencil_resolve_modes: RawVkResolveModeFlags,
    pub independent_resolve_none: u32,
    pub independent_resolve: u32,
    pub filter_minmax_single_component_formats: u32,
    pub filter_minmax_image_component_mapping: u32,
    pub max_timeline_semaphore_value_difference: u64,
    pub framebuffer_integer_color_sample_counts: RawVkSampleCountFlags,
}

impl VkRawType<VkPhysicalDeviceVulkan12Properties> for RawVkPhysicalDeviceVulkan12Properties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceVulkan12Properties) -> VkPhysicalDeviceVulkan12Properties {
        VkPhysicalDeviceVulkan12Properties {
            driver_id: RawVkDriverId::vk_to_wrapped(&src.driver_id),
            driver_name: new_string(&src.driver_name[0] as *const c_char),
            driver_info: new_string(&src.driver_info[0] as *const c_char),
            conformance_version: RawVkConformanceVersion::vk_to_wrapped(&src.conformance_version),
            denorm_behavior_independence: RawVkShaderFloatControlsIndependence::vk_to_wrapped(&src.denorm_behavior_independence),
            rounding_mode_independence: RawVkShaderFloatControlsIndependence::vk_to_wrapped(&src.rounding_mode_independence),
            shader_signed_zero_inf_nan_preserve_float_16: u32::vk_to_wrapped(&src.shader_signed_zero_inf_nan_preserve_float_16),
            shader_signed_zero_inf_nan_preserve_float_32: u32::vk_to_wrapped(&src.shader_signed_zero_inf_nan_preserve_float_32),
            shader_signed_zero_inf_nan_preserve_float_64: u32::vk_to_wrapped(&src.shader_signed_zero_inf_nan_preserve_float_64),
            shader_denorm_preserve_float_16: u32::vk_to_wrapped(&src.shader_denorm_preserve_float_16),
            shader_denorm_preserve_float_32: u32::vk_to_wrapped(&src.shader_denorm_preserve_float_32),
            shader_denorm_preserve_float_64: u32::vk_to_wrapped(&src.shader_denorm_preserve_float_64),
            shader_denorm_flush_to_zero_float_16: u32::vk_to_wrapped(&src.shader_denorm_flush_to_zero_float_16),
            shader_denorm_flush_to_zero_float_32: u32::vk_to_wrapped(&src.shader_denorm_flush_to_zero_float_32),
            shader_denorm_flush_to_zero_float_64: u32::vk_to_wrapped(&src.shader_denorm_flush_to_zero_float_64),
            shader_rounding_mode_rtefloat_16: u32::vk_to_wrapped(&src.shader_rounding_mode_rtefloat_16),
            shader_rounding_mode_rtefloat_32: u32::vk_to_wrapped(&src.shader_rounding_mode_rtefloat_32),
            shader_rounding_mode_rtefloat_64: u32::vk_to_wrapped(&src.shader_rounding_mode_rtefloat_64),
            shader_rounding_mode_rtzfloat_16: u32::vk_to_wrapped(&src.shader_rounding_mode_rtzfloat_16),
            shader_rounding_mode_rtzfloat_32: u32::vk_to_wrapped(&src.shader_rounding_mode_rtzfloat_32),
            shader_rounding_mode_rtzfloat_64: u32::vk_to_wrapped(&src.shader_rounding_mode_rtzfloat_64),
            max_update_after_bind_descriptors_in_all_pools: u32::vk_to_wrapped(&src.max_update_after_bind_descriptors_in_all_pools),
            shader_uniform_buffer_array_non_uniform_indexing_native: u32::vk_to_wrapped(&src.shader_uniform_buffer_array_non_uniform_indexing_native),
            shader_sampled_image_array_non_uniform_indexing_native: u32::vk_to_wrapped(&src.shader_sampled_image_array_non_uniform_indexing_native),
            shader_storage_buffer_array_non_uniform_indexing_native: u32::vk_to_wrapped(&src.shader_storage_buffer_array_non_uniform_indexing_native),
            shader_storage_image_array_non_uniform_indexing_native: u32::vk_to_wrapped(&src.shader_storage_image_array_non_uniform_indexing_native),
            shader_input_attachment_array_non_uniform_indexing_native: u32::vk_to_wrapped(&src.shader_input_attachment_array_non_uniform_indexing_native),
            robust_buffer_access_update_after_bind: u32::vk_to_wrapped(&src.robust_buffer_access_update_after_bind),
            quad_divergent_implicit_lod: u32::vk_to_wrapped(&src.quad_divergent_implicit_lod),
            max_per_stage_descriptor_update_after_bind_samplers: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_samplers),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_uniform_buffers),
            max_per_stage_descriptor_update_after_bind_storage_buffers: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_storage_buffers),
            max_per_stage_descriptor_update_after_bind_sampled_images: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_sampled_images),
            max_per_stage_descriptor_update_after_bind_storage_images: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_storage_images),
            max_per_stage_descriptor_update_after_bind_input_attachments: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_input_attachments),
            max_per_stage_update_after_bind_resources: u32::vk_to_wrapped(&src.max_per_stage_update_after_bind_resources),
            max_descriptor_set_update_after_bind_samplers: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_samplers),
            max_descriptor_set_update_after_bind_uniform_buffers: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_uniform_buffers),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_uniform_buffers_dynamic),
            max_descriptor_set_update_after_bind_storage_buffers: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_storage_buffers),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_storage_buffers_dynamic),
            max_descriptor_set_update_after_bind_sampled_images: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_sampled_images),
            max_descriptor_set_update_after_bind_storage_images: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_storage_images),
            max_descriptor_set_update_after_bind_input_attachments: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_input_attachments),
            supported_depth_resolve_modes: RawVkResolveModeFlags::vk_to_wrapped(&src.supported_depth_resolve_modes),
            supported_stencil_resolve_modes: RawVkResolveModeFlags::vk_to_wrapped(&src.supported_stencil_resolve_modes),
            independent_resolve_none: u32::vk_to_wrapped(&src.independent_resolve_none),
            independent_resolve: u32::vk_to_wrapped(&src.independent_resolve),
            filter_minmax_single_component_formats: u32::vk_to_wrapped(&src.filter_minmax_single_component_formats),
            filter_minmax_image_component_mapping: u32::vk_to_wrapped(&src.filter_minmax_image_component_mapping),
            max_timeline_semaphore_value_difference: u64::vk_to_wrapped(&src.max_timeline_semaphore_value_difference),
            framebuffer_integer_color_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.framebuffer_integer_color_sample_counts),
        }
    }
}

impl VkSetup for VkPhysicalDeviceVulkan12Properties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.conformance_version, fn_table);
    }
}

impl VkFree for RawVkPhysicalDeviceVulkan12Properties {
    fn vk_free(&self) {
        
    }
}