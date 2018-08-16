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
use vk::vk_structure_type::*;
use vk::vk_image_aspect_flags::*;

#[repr(C)]
pub struct RawVkBindImagePlaneMemoryInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    plane_aspect: RawVkImageAspectFlags,
}

#[derive(Debug, Clone)]
pub struct VkBindImagePlaneMemoryInfo {
    pub plane_aspect: VkImageAspectFlags,
}

impl VkRawType<VkBindImagePlaneMemoryInfo> for RawVkBindImagePlaneMemoryInfo {
    fn vk_to_wrapped(src: &RawVkBindImagePlaneMemoryInfo) -> VkBindImagePlaneMemoryInfo {
        VkBindImagePlaneMemoryInfo {
            plane_aspect: RawVkImageAspectFlags::vk_to_wrapped(&src.plane_aspect),
        }
    }
}

impl VkWrappedType<RawVkBindImagePlaneMemoryInfo> for VkBindImagePlaneMemoryInfo {
    fn vk_to_raw(src: &VkBindImagePlaneMemoryInfo, dst: &mut RawVkBindImagePlaneMemoryInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindImagePlaneMemoryInfo);
        dst.next = ptr::null();
        dst.plane_aspect = vk_to_raw_value(&src.plane_aspect);
    }
}

impl Default for VkBindImagePlaneMemoryInfo {
    fn default() -> VkBindImagePlaneMemoryInfo {
        VkBindImagePlaneMemoryInfo {
            plane_aspect: VkImageAspectFlags::default(),
        }
    }
}

impl VkSetup for VkBindImagePlaneMemoryInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBindImagePlaneMemoryInfo {
    fn vk_free(&mut self) {
        
    }
}