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
use vk::vk_subpass_contents::*;

#[derive(Debug, Clone)]
pub struct VkSubpassBeginInfo {
    pub contents: VkSubpassContents,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassBeginInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub contents: RawVkSubpassContents,
}

impl VkWrappedType<RawVkSubpassBeginInfo> for VkSubpassBeginInfo {
    fn vk_to_raw(src: &VkSubpassBeginInfo, dst: &mut RawVkSubpassBeginInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassBeginInfoKhr);
        dst.next = ptr::null();
        dst.contents = vk_to_raw_value(&src.contents);
    }
}

impl VkRawType<VkSubpassBeginInfo> for RawVkSubpassBeginInfo {
    fn vk_to_wrapped(src: &RawVkSubpassBeginInfo) -> VkSubpassBeginInfo {
        VkSubpassBeginInfo {
            contents: RawVkSubpassContents::vk_to_wrapped(&src.contents),
        }
    }
}

impl Default for VkSubpassBeginInfo {
    fn default() -> VkSubpassBeginInfo {
        VkSubpassBeginInfo {
            contents: VkSubpassContents::default(),
        }
    }
}

impl VkSetup for VkSubpassBeginInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassBeginInfo {
    fn vk_free(&mut self) {
        
    }
}