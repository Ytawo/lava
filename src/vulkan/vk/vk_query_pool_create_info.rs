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
use vulkan::vk::{VkQueryPoolCreateFlags,RawVkQueryPoolCreateFlags};
use vulkan::vk::{VkQueryType,RawVkQueryType};
use vulkan::vk::{VkQueryPipelineStatisticFlags,RawVkQueryPipelineStatisticFlags};

/// Wrapper for [VkQueryPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPoolCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkQueryPoolCreateInfo {
    pub flags: VkQueryPoolCreateFlags,
    pub query_type: VkQueryType,
    pub query_count: usize,
    pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkQueryPoolCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkQueryPoolCreateFlags,
    pub query_type: RawVkQueryType,
    pub query_count: u32,
    pub pipeline_statistics: RawVkQueryPipelineStatisticFlags,
}

impl VkWrappedType<RawVkQueryPoolCreateInfo> for VkQueryPoolCreateInfo {
    fn vk_to_raw(src: &VkQueryPoolCreateInfo, dst: &mut RawVkQueryPoolCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::QueryPoolCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.query_type = vk_to_raw_value(&src.query_type);
        dst.query_count = vk_to_raw_value(&src.query_count);
        dst.pipeline_statistics = vk_to_raw_value(&src.pipeline_statistics);
    }
}

impl VkRawType<VkQueryPoolCreateInfo> for RawVkQueryPoolCreateInfo {
    fn vk_to_wrapped(src: &RawVkQueryPoolCreateInfo) -> VkQueryPoolCreateInfo {
        VkQueryPoolCreateInfo {
            flags: RawVkQueryPoolCreateFlags::vk_to_wrapped(&src.flags),
            query_type: RawVkQueryType::vk_to_wrapped(&src.query_type),
            query_count: u32::vk_to_wrapped(&src.query_count),
            pipeline_statistics: RawVkQueryPipelineStatisticFlags::vk_to_wrapped(&src.pipeline_statistics),
        }
    }
}

impl Default for VkQueryPoolCreateInfo {
    fn default() -> VkQueryPoolCreateInfo {
        VkQueryPoolCreateInfo {
            flags: VkQueryPoolCreateFlags::default(),
            query_type: VkQueryType::default(),
            query_count: 0,
            pipeline_statistics: VkQueryPipelineStatisticFlags::default(),
        }
    }
}

impl VkSetup for VkQueryPoolCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkQueryPoolCreateInfo {
    fn vk_free(&mut self) {
        
    }
}