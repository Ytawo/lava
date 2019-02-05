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
use vulkan::amd::{VkRasterizationOrder,RawVkRasterizationOrder};

/// Wrapper for [VkPipelineRasterizationStateRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html).
#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationStateRasterizationOrder {
    pub rasterization_order: VkRasterizationOrder,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineRasterizationStateRasterizationOrder {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub rasterization_order: RawVkRasterizationOrder,
}

impl VkWrappedType<RawVkPipelineRasterizationStateRasterizationOrder> for VkPipelineRasterizationStateRasterizationOrder {
    fn vk_to_raw(src: &VkPipelineRasterizationStateRasterizationOrder, dst: &mut RawVkPipelineRasterizationStateRasterizationOrder) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineRasterizationStateRasterizationOrderAmd);
        dst.next = ptr::null();
        dst.rasterization_order = vk_to_raw_value(&src.rasterization_order);
    }
}

impl VkRawType<VkPipelineRasterizationStateRasterizationOrder> for RawVkPipelineRasterizationStateRasterizationOrder {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationStateRasterizationOrder) -> VkPipelineRasterizationStateRasterizationOrder {
        VkPipelineRasterizationStateRasterizationOrder {
            rasterization_order: RawVkRasterizationOrder::vk_to_wrapped(&src.rasterization_order),
        }
    }
}

impl Default for VkPipelineRasterizationStateRasterizationOrder {
    fn default() -> VkPipelineRasterizationStateRasterizationOrder {
        VkPipelineRasterizationStateRasterizationOrder {
            rasterization_order: VkRasterizationOrder::default(),
        }
    }
}

impl VkSetup for VkPipelineRasterizationStateRasterizationOrder {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineRasterizationStateRasterizationOrder {
    fn vk_free(&mut self) {
        
    }
}