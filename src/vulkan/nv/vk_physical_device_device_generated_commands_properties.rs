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

/// Wrapper for [VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsProperties {
    pub max_graphics_shader_group_count: usize,
    pub max_indirect_sequence_count: usize,
    pub max_indirect_commands_token_count: usize,
    pub max_indirect_commands_stream_count: usize,
    pub max_indirect_commands_token_offset: usize,
    pub max_indirect_commands_stream_stride: usize,
    pub min_sequences_count_buffer_offset_alignment: usize,
    pub min_sequences_index_buffer_offset_alignment: usize,
    pub min_indirect_commands_buffer_offset_alignment: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDeviceGeneratedCommandsProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub max_graphics_shader_group_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_stream_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_stream_stride: u32,
    pub min_sequences_count_buffer_offset_alignment: u32,
    pub min_sequences_index_buffer_offset_alignment: u32,
    pub min_indirect_commands_buffer_offset_alignment: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDeviceGeneratedCommandsProperties> for VkPhysicalDeviceDeviceGeneratedCommandsProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceDeviceGeneratedCommandsProperties, dst: &mut RawVkPhysicalDeviceDeviceGeneratedCommandsProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDeviceGeneratedCommandsPropertiesNv);
        dst.next = ptr::null_mut();
        dst.max_graphics_shader_group_count = vk_to_raw_value(&src.max_graphics_shader_group_count);
        dst.max_indirect_sequence_count = vk_to_raw_value(&src.max_indirect_sequence_count);
        dst.max_indirect_commands_token_count = vk_to_raw_value(&src.max_indirect_commands_token_count);
        dst.max_indirect_commands_stream_count = vk_to_raw_value(&src.max_indirect_commands_stream_count);
        dst.max_indirect_commands_token_offset = vk_to_raw_value(&src.max_indirect_commands_token_offset);
        dst.max_indirect_commands_stream_stride = vk_to_raw_value(&src.max_indirect_commands_stream_stride);
        dst.min_sequences_count_buffer_offset_alignment = vk_to_raw_value(&src.min_sequences_count_buffer_offset_alignment);
        dst.min_sequences_index_buffer_offset_alignment = vk_to_raw_value(&src.min_sequences_index_buffer_offset_alignment);
        dst.min_indirect_commands_buffer_offset_alignment = vk_to_raw_value(&src.min_indirect_commands_buffer_offset_alignment);
    }
}

impl VkRawType<VkPhysicalDeviceDeviceGeneratedCommandsProperties> for RawVkPhysicalDeviceDeviceGeneratedCommandsProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDeviceGeneratedCommandsProperties) -> VkPhysicalDeviceDeviceGeneratedCommandsProperties {
        VkPhysicalDeviceDeviceGeneratedCommandsProperties {
            max_graphics_shader_group_count: u32::vk_to_wrapped(&src.max_graphics_shader_group_count),
            max_indirect_sequence_count: u32::vk_to_wrapped(&src.max_indirect_sequence_count),
            max_indirect_commands_token_count: u32::vk_to_wrapped(&src.max_indirect_commands_token_count),
            max_indirect_commands_stream_count: u32::vk_to_wrapped(&src.max_indirect_commands_stream_count),
            max_indirect_commands_token_offset: u32::vk_to_wrapped(&src.max_indirect_commands_token_offset),
            max_indirect_commands_stream_stride: u32::vk_to_wrapped(&src.max_indirect_commands_stream_stride),
            min_sequences_count_buffer_offset_alignment: u32::vk_to_wrapped(&src.min_sequences_count_buffer_offset_alignment),
            min_sequences_index_buffer_offset_alignment: u32::vk_to_wrapped(&src.min_sequences_index_buffer_offset_alignment),
            min_indirect_commands_buffer_offset_alignment: u32::vk_to_wrapped(&src.min_indirect_commands_buffer_offset_alignment),
        }
    }
}

impl Default for VkPhysicalDeviceDeviceGeneratedCommandsProperties {
    fn default() -> VkPhysicalDeviceDeviceGeneratedCommandsProperties {
        VkPhysicalDeviceDeviceGeneratedCommandsProperties {
            max_graphics_shader_group_count: 0,
            max_indirect_sequence_count: 0,
            max_indirect_commands_token_count: 0,
            max_indirect_commands_stream_count: 0,
            max_indirect_commands_token_offset: 0,
            max_indirect_commands_stream_stride: 0,
            min_sequences_count_buffer_offset_alignment: 0,
            min_sequences_index_buffer_offset_alignment: 0,
            min_indirect_commands_buffer_offset_alignment: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDeviceGeneratedCommandsProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDeviceGeneratedCommandsProperties {
    fn vk_free(&self) {
        
    }
}