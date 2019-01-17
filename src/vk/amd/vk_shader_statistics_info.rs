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
use vk::vk_shader_stage_flags::*;
use vk::amd::vk_shader_resource_usage::*;

#[derive(Debug, Clone)]
pub struct VkShaderStatisticsInfo {
    pub shader_stage_mask: VkShaderStageFlags,
    pub resource_usage: VkShaderResourceUsage,
    pub num_physical_vgprs: usize,
    pub num_physical_sgprs: usize,
    pub num_available_vgprs: usize,
    pub num_available_sgprs: usize,
    pub compute_work_group_size: [usize; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkShaderStatisticsInfo {
    pub shader_stage_mask: RawVkShaderStageFlags,
    pub resource_usage: RawVkShaderResourceUsage,
    pub num_physical_vgprs: u32,
    pub num_physical_sgprs: u32,
    pub num_available_vgprs: u32,
    pub num_available_sgprs: u32,
    pub compute_work_group_size: [u32; 3],
}

impl VkWrappedType<RawVkShaderStatisticsInfo> for VkShaderStatisticsInfo {
    fn vk_to_raw(src: &VkShaderStatisticsInfo, dst: &mut RawVkShaderStatisticsInfo) {
        dst.shader_stage_mask = vk_to_raw_value(&src.shader_stage_mask);
        dst.resource_usage = vk_to_raw_value(&src.resource_usage);
        dst.num_physical_vgprs = vk_to_raw_value(&src.num_physical_vgprs);
        dst.num_physical_sgprs = vk_to_raw_value(&src.num_physical_sgprs);
        dst.num_available_vgprs = vk_to_raw_value(&src.num_available_vgprs);
        dst.num_available_sgprs = vk_to_raw_value(&src.num_available_sgprs);
        dst.compute_work_group_size = unsafe { let mut dst_array : [u32; 3] = mem::uninitialized(); vk_to_raw_array(&src.compute_work_group_size, &mut dst_array); dst_array };
    }
}

impl VkRawType<VkShaderStatisticsInfo> for RawVkShaderStatisticsInfo {
    fn vk_to_wrapped(src: &RawVkShaderStatisticsInfo) -> VkShaderStatisticsInfo {
        VkShaderStatisticsInfo {
            shader_stage_mask: RawVkShaderStageFlags::vk_to_wrapped(&src.shader_stage_mask),
            resource_usage: RawVkShaderResourceUsage::vk_to_wrapped(&src.resource_usage),
            num_physical_vgprs: u32::vk_to_wrapped(&src.num_physical_vgprs),
            num_physical_sgprs: u32::vk_to_wrapped(&src.num_physical_sgprs),
            num_available_vgprs: u32::vk_to_wrapped(&src.num_available_vgprs),
            num_available_sgprs: u32::vk_to_wrapped(&src.num_available_sgprs),
            compute_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::uninitialized(); vk_to_wrapped_array(&src.compute_work_group_size, &mut dst_array); dst_array },
        }
    }
}

impl Default for VkShaderStatisticsInfo {
    fn default() -> VkShaderStatisticsInfo {
        VkShaderStatisticsInfo {
            shader_stage_mask: VkShaderStageFlags::default(),
            resource_usage: VkShaderResourceUsage::default(),
            num_physical_vgprs: 0,
            num_physical_sgprs: 0,
            num_available_vgprs: 0,
            num_available_sgprs: 0,
            compute_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
        }
    }
}

impl VkSetup for VkShaderStatisticsInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.resource_usage, fn_table, instance, device);
    }
}

impl VkFree for RawVkShaderStatisticsInfo {
    fn vk_free(&mut self) {
        RawVkShaderResourceUsage::vk_free(&mut self.resource_usage);
    }
}