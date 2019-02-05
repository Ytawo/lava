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
use vulkan::nv::{VkPipelineCoverageModulationStateCreateFlags,RawVkPipelineCoverageModulationStateCreateFlags};
use vulkan::nv::{VkCoverageModulationMode,RawVkCoverageModulationMode};

/// Wrapper for [VkPipelineCoverageModulationStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkPipelineCoverageModulationStateCreateInfo {
    pub flags: VkPipelineCoverageModulationStateCreateFlags,
    pub coverage_modulation_mode: VkCoverageModulationMode,
    pub coverage_modulation_table_enable: bool,
    pub coverage_modulation_table: Option<Vec<f32>>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineCoverageModulationStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkPipelineCoverageModulationStateCreateFlags,
    pub coverage_modulation_mode: RawVkCoverageModulationMode,
    pub coverage_modulation_table_enable: u32,
    pub coverage_modulation_table_count: u32,
    pub coverage_modulation_table: *mut f32,
}

impl VkWrappedType<RawVkPipelineCoverageModulationStateCreateInfo> for VkPipelineCoverageModulationStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineCoverageModulationStateCreateInfo, dst: &mut RawVkPipelineCoverageModulationStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineCoverageModulationStateCreateInfoNv);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.coverage_modulation_mode = vk_to_raw_value(&src.coverage_modulation_mode);
        dst.coverage_modulation_table_enable = vk_to_raw_value(&src.coverage_modulation_table_enable);
        dst.coverage_modulation_table_count = get_array_option_len(&src.coverage_modulation_table) as u32;
        dst.coverage_modulation_table = get_vec_ptr_checked(&src.coverage_modulation_table);
    }
}

impl VkRawType<VkPipelineCoverageModulationStateCreateInfo> for RawVkPipelineCoverageModulationStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineCoverageModulationStateCreateInfo) -> VkPipelineCoverageModulationStateCreateInfo {
        VkPipelineCoverageModulationStateCreateInfo {
            flags: RawVkPipelineCoverageModulationStateCreateFlags::vk_to_wrapped(&src.flags),
            coverage_modulation_mode: RawVkCoverageModulationMode::vk_to_wrapped(&src.coverage_modulation_mode),
            coverage_modulation_table_enable: u32::vk_to_wrapped(&src.coverage_modulation_table_enable),
            coverage_modulation_table: vec_from_ptr_checked(src.coverage_modulation_table_count as usize, src.coverage_modulation_table),
        }
    }
}

impl Default for VkPipelineCoverageModulationStateCreateInfo {
    fn default() -> VkPipelineCoverageModulationStateCreateInfo {
        VkPipelineCoverageModulationStateCreateInfo {
            flags: Default::default(),
            coverage_modulation_mode: Default::default(),
            coverage_modulation_table_enable: false,
            coverage_modulation_table: None,
        }
    }
}

impl VkSetup for VkPipelineCoverageModulationStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineCoverageModulationStateCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.coverage_modulation_table);
    }
}