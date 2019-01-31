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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::vk_fence_create_flags::*;

#[derive(Debug, Clone)]
pub struct VkFenceCreateInfo {
    pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkFenceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkFenceCreateFlags,
}

impl VkWrappedType<RawVkFenceCreateInfo> for VkFenceCreateInfo {
    fn vk_to_raw(src: &VkFenceCreateInfo, dst: &mut RawVkFenceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FenceCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl VkRawType<VkFenceCreateInfo> for RawVkFenceCreateInfo {
    fn vk_to_wrapped(src: &RawVkFenceCreateInfo) -> VkFenceCreateInfo {
        VkFenceCreateInfo {
            flags: RawVkFenceCreateFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl Default for VkFenceCreateInfo {
    fn default() -> VkFenceCreateInfo {
        VkFenceCreateInfo {
            flags: VkFenceCreateFlags::default(),
        }
    }
}

impl VkSetup for VkFenceCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkFenceCreateInfo {
    fn vk_free(&mut self) {
        
    }
}