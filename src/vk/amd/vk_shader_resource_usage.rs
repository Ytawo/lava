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

#[derive(Debug, Clone)]
pub struct VkShaderResourceUsage {
    pub num_used_vgprs: usize,
    pub num_used_sgprs: usize,
    pub lds_size_per_local_work_group: usize,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkShaderResourceUsage {
    pub num_used_vgprs: u32,
    pub num_used_sgprs: u32,
    pub lds_size_per_local_work_group: u32,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}

impl VkWrappedType<RawVkShaderResourceUsage> for VkShaderResourceUsage {
    fn vk_to_raw(src: &VkShaderResourceUsage, dst: &mut RawVkShaderResourceUsage) {
        dst.num_used_vgprs = vk_to_raw_value(&src.num_used_vgprs);
        dst.num_used_sgprs = vk_to_raw_value(&src.num_used_sgprs);
        dst.lds_size_per_local_work_group = vk_to_raw_value(&src.lds_size_per_local_work_group);
        dst.lds_usage_size_in_bytes = src.lds_usage_size_in_bytes;
        dst.scratch_mem_usage_in_bytes = src.scratch_mem_usage_in_bytes;
    }
}

impl VkRawType<VkShaderResourceUsage> for RawVkShaderResourceUsage {
    fn vk_to_wrapped(src: &RawVkShaderResourceUsage) -> VkShaderResourceUsage {
        VkShaderResourceUsage {
            num_used_vgprs: u32::vk_to_wrapped(&src.num_used_vgprs),
            num_used_sgprs: u32::vk_to_wrapped(&src.num_used_sgprs),
            lds_size_per_local_work_group: u32::vk_to_wrapped(&src.lds_size_per_local_work_group),
            lds_usage_size_in_bytes: src.lds_usage_size_in_bytes,
            scratch_mem_usage_in_bytes: src.scratch_mem_usage_in_bytes,
        }
    }
}

impl Default for VkShaderResourceUsage {
    fn default() -> VkShaderResourceUsage {
        VkShaderResourceUsage {
            num_used_vgprs: 0,
            num_used_sgprs: 0,
            lds_size_per_local_work_group: 0,
            lds_usage_size_in_bytes: 0,
            scratch_mem_usage_in_bytes: 0,
        }
    }
}

impl VkSetup for VkShaderResourceUsage {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkShaderResourceUsage {
    fn vk_free(&mut self) {
        
    }
}