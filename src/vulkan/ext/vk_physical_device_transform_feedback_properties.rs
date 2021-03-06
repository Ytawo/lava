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

/// Wrapper for [VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceTransformFeedbackProperties {
    pub max_transform_feedback_streams: usize,
    pub max_transform_feedback_buffers: usize,
    pub max_transform_feedback_buffer_size: usize,
    pub max_transform_feedback_stream_data_size: usize,
    pub max_transform_feedback_buffer_data_size: usize,
    pub max_transform_feedback_buffer_data_stride: usize,
    pub transform_feedback_queries: bool,
    pub transform_feedback_streams_lines_triangles: bool,
    pub transform_feedback_rasterization_stream_select: bool,
    pub transform_feedback_draw: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceTransformFeedbackProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub max_transform_feedback_streams: u32,
    pub max_transform_feedback_buffers: u32,
    pub max_transform_feedback_buffer_size: u64,
    pub max_transform_feedback_stream_data_size: u32,
    pub max_transform_feedback_buffer_data_size: u32,
    pub max_transform_feedback_buffer_data_stride: u32,
    pub transform_feedback_queries: u32,
    pub transform_feedback_streams_lines_triangles: u32,
    pub transform_feedback_rasterization_stream_select: u32,
    pub transform_feedback_draw: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceTransformFeedbackProperties> for VkPhysicalDeviceTransformFeedbackProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceTransformFeedbackProperties, dst: &mut RawVkPhysicalDeviceTransformFeedbackProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceTransformFeedbackPropertiesExt);
        dst.next = ptr::null_mut();
        dst.max_transform_feedback_streams = vk_to_raw_value(&src.max_transform_feedback_streams);
        dst.max_transform_feedback_buffers = vk_to_raw_value(&src.max_transform_feedback_buffers);
        dst.max_transform_feedback_buffer_size = vk_to_raw_value(&src.max_transform_feedback_buffer_size);
        dst.max_transform_feedback_stream_data_size = vk_to_raw_value(&src.max_transform_feedback_stream_data_size);
        dst.max_transform_feedback_buffer_data_size = vk_to_raw_value(&src.max_transform_feedback_buffer_data_size);
        dst.max_transform_feedback_buffer_data_stride = vk_to_raw_value(&src.max_transform_feedback_buffer_data_stride);
        dst.transform_feedback_queries = vk_to_raw_value(&src.transform_feedback_queries);
        dst.transform_feedback_streams_lines_triangles = vk_to_raw_value(&src.transform_feedback_streams_lines_triangles);
        dst.transform_feedback_rasterization_stream_select = vk_to_raw_value(&src.transform_feedback_rasterization_stream_select);
        dst.transform_feedback_draw = vk_to_raw_value(&src.transform_feedback_draw);
    }
}

impl VkRawType<VkPhysicalDeviceTransformFeedbackProperties> for RawVkPhysicalDeviceTransformFeedbackProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceTransformFeedbackProperties) -> VkPhysicalDeviceTransformFeedbackProperties {
        VkPhysicalDeviceTransformFeedbackProperties {
            max_transform_feedback_streams: u32::vk_to_wrapped(&src.max_transform_feedback_streams),
            max_transform_feedback_buffers: u32::vk_to_wrapped(&src.max_transform_feedback_buffers),
            max_transform_feedback_buffer_size: u64::vk_to_wrapped(&src.max_transform_feedback_buffer_size),
            max_transform_feedback_stream_data_size: u32::vk_to_wrapped(&src.max_transform_feedback_stream_data_size),
            max_transform_feedback_buffer_data_size: u32::vk_to_wrapped(&src.max_transform_feedback_buffer_data_size),
            max_transform_feedback_buffer_data_stride: u32::vk_to_wrapped(&src.max_transform_feedback_buffer_data_stride),
            transform_feedback_queries: u32::vk_to_wrapped(&src.transform_feedback_queries),
            transform_feedback_streams_lines_triangles: u32::vk_to_wrapped(&src.transform_feedback_streams_lines_triangles),
            transform_feedback_rasterization_stream_select: u32::vk_to_wrapped(&src.transform_feedback_rasterization_stream_select),
            transform_feedback_draw: u32::vk_to_wrapped(&src.transform_feedback_draw),
        }
    }
}

impl Default for VkPhysicalDeviceTransformFeedbackProperties {
    fn default() -> VkPhysicalDeviceTransformFeedbackProperties {
        VkPhysicalDeviceTransformFeedbackProperties {
            max_transform_feedback_streams: 0,
            max_transform_feedback_buffers: 0,
            max_transform_feedback_buffer_size: 0,
            max_transform_feedback_stream_data_size: 0,
            max_transform_feedback_buffer_data_size: 0,
            max_transform_feedback_buffer_data_stride: 0,
            transform_feedback_queries: false,
            transform_feedback_streams_lines_triangles: false,
            transform_feedback_rasterization_stream_select: false,
            transform_feedback_draw: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceTransformFeedbackProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceTransformFeedbackProperties {
    fn vk_free(&self) {
        
    }
}