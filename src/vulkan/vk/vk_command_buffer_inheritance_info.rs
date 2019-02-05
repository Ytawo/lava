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
use vulkan::vk::{VkRenderPass,RawVkRenderPass};
use vulkan::vk::{VkFramebuffer,RawVkFramebuffer};
use vulkan::vk::{VkQueryControlFlags,RawVkQueryControlFlags};
use vulkan::vk::{VkQueryPipelineStatisticFlags,RawVkQueryPipelineStatisticFlags};

/// Wrapper for [VkCommandBufferInheritanceInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferInheritanceInfo.html).
#[derive(Debug, Clone)]
pub struct VkCommandBufferInheritanceInfo {
    pub render_pass: VkRenderPass,
    pub subpass: usize,
    pub framebuffer: VkFramebuffer,
    pub occlusion_query_enable: bool,
    pub query_flags: VkQueryControlFlags,
    pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandBufferInheritanceInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub render_pass: RawVkRenderPass,
    pub subpass: u32,
    pub framebuffer: RawVkFramebuffer,
    pub occlusion_query_enable: u32,
    pub query_flags: RawVkQueryControlFlags,
    pub pipeline_statistics: RawVkQueryPipelineStatisticFlags,
}

impl VkWrappedType<RawVkCommandBufferInheritanceInfo> for VkCommandBufferInheritanceInfo {
    fn vk_to_raw(src: &VkCommandBufferInheritanceInfo, dst: &mut RawVkCommandBufferInheritanceInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandBufferInheritanceInfo);
        dst.next = ptr::null();
        dst.render_pass = vk_to_raw_value(&src.render_pass);
        dst.subpass = vk_to_raw_value(&src.subpass);
        dst.framebuffer = vk_to_raw_value(&src.framebuffer);
        dst.occlusion_query_enable = vk_to_raw_value(&src.occlusion_query_enable);
        dst.query_flags = vk_to_raw_value(&src.query_flags);
        dst.pipeline_statistics = vk_to_raw_value(&src.pipeline_statistics);
    }
}

impl VkRawType<VkCommandBufferInheritanceInfo> for RawVkCommandBufferInheritanceInfo {
    fn vk_to_wrapped(src: &RawVkCommandBufferInheritanceInfo) -> VkCommandBufferInheritanceInfo {
        VkCommandBufferInheritanceInfo {
            render_pass: RawVkRenderPass::vk_to_wrapped(&src.render_pass),
            subpass: u32::vk_to_wrapped(&src.subpass),
            framebuffer: RawVkFramebuffer::vk_to_wrapped(&src.framebuffer),
            occlusion_query_enable: u32::vk_to_wrapped(&src.occlusion_query_enable),
            query_flags: RawVkQueryControlFlags::vk_to_wrapped(&src.query_flags),
            pipeline_statistics: RawVkQueryPipelineStatisticFlags::vk_to_wrapped(&src.pipeline_statistics),
        }
    }
}

impl Default for VkCommandBufferInheritanceInfo {
    fn default() -> VkCommandBufferInheritanceInfo {
        VkCommandBufferInheritanceInfo {
            render_pass: Default::default(),
            subpass: 0,
            framebuffer: Default::default(),
            occlusion_query_enable: false,
            query_flags: Default::default(),
            pipeline_statistics: Default::default(),
        }
    }
}

impl VkSetup for VkCommandBufferInheritanceInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.render_pass, fn_table);
        VkSetup::vk_setup(&mut self.framebuffer, fn_table);
    }
}

impl VkFree for RawVkCommandBufferInheritanceInfo {
    fn vk_free(&self) {
        
    }
}