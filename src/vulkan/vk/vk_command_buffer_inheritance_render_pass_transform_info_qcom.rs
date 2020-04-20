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
use vulkan::khr::{VkSurfaceTransformFlags,RawVkSurfaceTransformFlags};
use vulkan::vk::{VkRect2D,RawVkRect2D};

/// Wrapper for [VkCommandBufferInheritanceRenderPassTransformInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html).
#[derive(Debug, Clone)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub transform: VkSurfaceTransformFlags,
    pub render_area: VkRect2D,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub transform: RawVkSurfaceTransformFlags,
    pub render_area: RawVkRect2D,
}

impl VkWrappedType<RawVkCommandBufferInheritanceRenderPassTransformInfoQCOM> for VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn vk_to_raw(src: &VkCommandBufferInheritanceRenderPassTransformInfoQCOM, dst: &mut RawVkCommandBufferInheritanceRenderPassTransformInfoQCOM) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandBufferInheritanceRenderPassTransformInfoQcom);
        dst.next = ptr::null_mut();
        dst.transform = vk_to_raw_value(&src.transform);
        dst.render_area = vk_to_raw_value(&src.render_area);
    }
}

impl VkRawType<VkCommandBufferInheritanceRenderPassTransformInfoQCOM> for RawVkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn vk_to_wrapped(src: &RawVkCommandBufferInheritanceRenderPassTransformInfoQCOM) -> VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
        VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
            transform: RawVkSurfaceTransformFlags::vk_to_wrapped(&src.transform),
            render_area: RawVkRect2D::vk_to_wrapped(&src.render_area),
        }
    }
}

impl Default for VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn default() -> VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
        VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
            transform: Default::default(),
            render_area: Default::default(),
        }
    }
}

impl VkSetup for VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.render_area, fn_table);
    }
}

impl VkFree for RawVkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn vk_free(&self) {
        
    }
}