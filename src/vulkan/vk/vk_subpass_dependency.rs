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
use vulkan::vk::{VkPipelineStageFlags,RawVkPipelineStageFlags};
use vulkan::vk::{VkAccessFlags,RawVkAccessFlags};
use vulkan::vk::{VkDependencyFlags,RawVkDependencyFlags};

/// Wrapper for [VkSubpassDependency](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSubpassDependency.html).
#[derive(Debug, Clone)]
pub struct VkSubpassDependency {
    pub src_subpass: usize,
    pub dst_subpass: usize,
    pub src_stage_mask: VkPipelineStageFlags,
    pub dst_stage_mask: VkPipelineStageFlags,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub dependency_flags: VkDependencyFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: RawVkPipelineStageFlags,
    pub dst_stage_mask: RawVkPipelineStageFlags,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
    pub dependency_flags: RawVkDependencyFlags,
}

impl VkWrappedType<RawVkSubpassDependency> for VkSubpassDependency {
    fn vk_to_raw(src: &VkSubpassDependency, dst: &mut RawVkSubpassDependency) {
        dst.src_subpass = vk_to_raw_value(&src.src_subpass);
        dst.dst_subpass = vk_to_raw_value(&src.dst_subpass);
        dst.src_stage_mask = vk_to_raw_value(&src.src_stage_mask);
        dst.dst_stage_mask = vk_to_raw_value(&src.dst_stage_mask);
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
        dst.dependency_flags = vk_to_raw_value(&src.dependency_flags);
    }
}

impl VkRawType<VkSubpassDependency> for RawVkSubpassDependency {
    fn vk_to_wrapped(src: &RawVkSubpassDependency) -> VkSubpassDependency {
        VkSubpassDependency {
            src_subpass: u32::vk_to_wrapped(&src.src_subpass),
            dst_subpass: u32::vk_to_wrapped(&src.dst_subpass),
            src_stage_mask: RawVkPipelineStageFlags::vk_to_wrapped(&src.src_stage_mask),
            dst_stage_mask: RawVkPipelineStageFlags::vk_to_wrapped(&src.dst_stage_mask),
            src_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.src_access_mask),
            dst_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.dst_access_mask),
            dependency_flags: RawVkDependencyFlags::vk_to_wrapped(&src.dependency_flags),
        }
    }
}

impl Default for VkSubpassDependency {
    fn default() -> VkSubpassDependency {
        VkSubpassDependency {
            src_subpass: 0,
            dst_subpass: 0,
            src_stage_mask: VkPipelineStageFlags::default(),
            dst_stage_mask: VkPipelineStageFlags::default(),
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
            dependency_flags: VkDependencyFlags::default(),
        }
    }
}

impl VkSetup for VkSubpassDependency {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassDependency {
    fn vk_free(&mut self) {
        
    }
}