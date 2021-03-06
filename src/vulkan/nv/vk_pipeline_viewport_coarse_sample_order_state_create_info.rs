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
use vulkan::nv::{VkCoarseSampleOrderType,RawVkCoarseSampleOrderType};
use vulkan::nv::{VkCoarseSampleOrderCustom,RawVkCoarseSampleOrderCustom};

/// Wrapper for [VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfo {
    pub sample_order_type: VkCoarseSampleOrderType,
    pub custom_sample_orders: Vec<VkCoarseSampleOrderCustom>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineViewportCoarseSampleOrderStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub sample_order_type: RawVkCoarseSampleOrderType,
    pub custom_sample_order_count: u32,
    pub custom_sample_orders: *mut RawVkCoarseSampleOrderCustom,
}

impl VkWrappedType<RawVkPipelineViewportCoarseSampleOrderStateCreateInfo> for VkPipelineViewportCoarseSampleOrderStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineViewportCoarseSampleOrderStateCreateInfo, dst: &mut RawVkPipelineViewportCoarseSampleOrderStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineViewportCoarseSampleOrderStateCreateInfoNv);
        dst.next = ptr::null_mut();
        dst.sample_order_type = vk_to_raw_value(&src.sample_order_type);
        dst.custom_sample_order_count = src.custom_sample_orders.len() as u32;
        dst.custom_sample_orders = new_ptr_vk_array(&src.custom_sample_orders);
    }
}

impl VkRawType<VkPipelineViewportCoarseSampleOrderStateCreateInfo> for RawVkPipelineViewportCoarseSampleOrderStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineViewportCoarseSampleOrderStateCreateInfo) -> VkPipelineViewportCoarseSampleOrderStateCreateInfo {
        VkPipelineViewportCoarseSampleOrderStateCreateInfo {
            sample_order_type: RawVkCoarseSampleOrderType::vk_to_wrapped(&src.sample_order_type),
            custom_sample_orders: new_vk_array(src.custom_sample_order_count, src.custom_sample_orders),
        }
    }
}

impl Default for VkPipelineViewportCoarseSampleOrderStateCreateInfo {
    fn default() -> VkPipelineViewportCoarseSampleOrderStateCreateInfo {
        VkPipelineViewportCoarseSampleOrderStateCreateInfo {
            sample_order_type: Default::default(),
            custom_sample_orders: Vec::new(),
        }
    }
}

impl VkSetup for VkPipelineViewportCoarseSampleOrderStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineViewportCoarseSampleOrderStateCreateInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.custom_sample_order_count as usize, self.custom_sample_orders);
    }
}