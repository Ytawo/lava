// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
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
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDescriptorIndexingProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
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
}

impl VkWrappedType<RawVkPhysicalDeviceDescriptorIndexingProperties> for VkPhysicalDeviceDescriptorIndexingProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceDescriptorIndexingProperties, dst: &mut RawVkPhysicalDeviceDescriptorIndexingProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDescriptorIndexingPropertiesExt);
        dst.next = ptr::null();
        dst.max_update_after_bind_descriptors_in_all_pools = vk_to_raw_value(&src.max_update_after_bind_descriptors_in_all_pools);
        dst.shader_uniform_buffer_array_non_uniform_indexing_native = vk_to_raw_value(&src.shader_uniform_buffer_array_non_uniform_indexing_native);
        dst.shader_sampled_image_array_non_uniform_indexing_native = vk_to_raw_value(&src.shader_sampled_image_array_non_uniform_indexing_native);
        dst.shader_storage_buffer_array_non_uniform_indexing_native = vk_to_raw_value(&src.shader_storage_buffer_array_non_uniform_indexing_native);
        dst.shader_storage_image_array_non_uniform_indexing_native = vk_to_raw_value(&src.shader_storage_image_array_non_uniform_indexing_native);
        dst.shader_input_attachment_array_non_uniform_indexing_native = vk_to_raw_value(&src.shader_input_attachment_array_non_uniform_indexing_native);
        dst.robust_buffer_access_update_after_bind = vk_to_raw_value(&src.robust_buffer_access_update_after_bind);
        dst.quad_divergent_implicit_lod = vk_to_raw_value(&src.quad_divergent_implicit_lod);
        dst.max_per_stage_descriptor_update_after_bind_samplers = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_samplers);
        dst.max_per_stage_descriptor_update_after_bind_uniform_buffers = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_uniform_buffers);
        dst.max_per_stage_descriptor_update_after_bind_storage_buffers = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_storage_buffers);
        dst.max_per_stage_descriptor_update_after_bind_sampled_images = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_sampled_images);
        dst.max_per_stage_descriptor_update_after_bind_storage_images = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_storage_images);
        dst.max_per_stage_descriptor_update_after_bind_input_attachments = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_input_attachments);
        dst.max_per_stage_update_after_bind_resources = vk_to_raw_value(&src.max_per_stage_update_after_bind_resources);
        dst.max_descriptor_set_update_after_bind_samplers = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_samplers);
        dst.max_descriptor_set_update_after_bind_uniform_buffers = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_uniform_buffers);
        dst.max_descriptor_set_update_after_bind_uniform_buffers_dynamic = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_uniform_buffers_dynamic);
        dst.max_descriptor_set_update_after_bind_storage_buffers = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_storage_buffers);
        dst.max_descriptor_set_update_after_bind_storage_buffers_dynamic = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_storage_buffers_dynamic);
        dst.max_descriptor_set_update_after_bind_sampled_images = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_sampled_images);
        dst.max_descriptor_set_update_after_bind_storage_images = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_storage_images);
        dst.max_descriptor_set_update_after_bind_input_attachments = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_input_attachments);
    }
}

impl VkRawType<VkPhysicalDeviceDescriptorIndexingProperties> for RawVkPhysicalDeviceDescriptorIndexingProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDescriptorIndexingProperties) -> VkPhysicalDeviceDescriptorIndexingProperties {
        VkPhysicalDeviceDescriptorIndexingProperties {
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
        }
    }
}

impl Default for VkPhysicalDeviceDescriptorIndexingProperties {
    fn default() -> VkPhysicalDeviceDescriptorIndexingProperties {
        VkPhysicalDeviceDescriptorIndexingProperties {
            max_update_after_bind_descriptors_in_all_pools: 0,
            shader_uniform_buffer_array_non_uniform_indexing_native: false,
            shader_sampled_image_array_non_uniform_indexing_native: false,
            shader_storage_buffer_array_non_uniform_indexing_native: false,
            shader_storage_image_array_non_uniform_indexing_native: false,
            shader_input_attachment_array_non_uniform_indexing_native: false,
            robust_buffer_access_update_after_bind: false,
            quad_divergent_implicit_lod: false,
            max_per_stage_descriptor_update_after_bind_samplers: 0,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: 0,
            max_per_stage_descriptor_update_after_bind_storage_buffers: 0,
            max_per_stage_descriptor_update_after_bind_sampled_images: 0,
            max_per_stage_descriptor_update_after_bind_storage_images: 0,
            max_per_stage_descriptor_update_after_bind_input_attachments: 0,
            max_per_stage_update_after_bind_resources: 0,
            max_descriptor_set_update_after_bind_samplers: 0,
            max_descriptor_set_update_after_bind_uniform_buffers: 0,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: 0,
            max_descriptor_set_update_after_bind_storage_buffers: 0,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: 0,
            max_descriptor_set_update_after_bind_sampled_images: 0,
            max_descriptor_set_update_after_bind_storage_images: 0,
            max_descriptor_set_update_after_bind_input_attachments: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDescriptorIndexingProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDescriptorIndexingProperties {
    fn vk_free(&mut self) {
        
    }
}