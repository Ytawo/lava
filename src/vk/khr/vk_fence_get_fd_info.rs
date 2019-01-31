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
use vk::vk_fence::*;
use vk::vk_external_fence_handle_type_flags::*;

#[derive(Debug, Clone)]
pub struct VkFenceGetFdInfo<'a> {
    pub fence: &'a VkFence,
    pub handle_type: VkExternalFenceHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkFenceGetFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub fence: RawVkFence,
    pub handle_type: RawVkExternalFenceHandleTypeFlags,
}

impl<'a> VkWrappedType<RawVkFenceGetFdInfo> for VkFenceGetFdInfo<'a> {
    fn vk_to_raw(src: &VkFenceGetFdInfo, dst: &mut RawVkFenceGetFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FenceGetFdInfoKhr);
        dst.next = ptr::null();
        dst.fence = vk_to_raw_value(src.fence);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl Default for VkFenceGetFdInfo<'static> {
    fn default() -> VkFenceGetFdInfo<'static> {
        VkFenceGetFdInfo {
            fence: vk_null_ref(),
            handle_type: VkExternalFenceHandleTypeFlags::default(),
        }
    }
}

impl<'a> VkSetup for VkFenceGetFdInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkFenceGetFdInfo {
    fn vk_free(&mut self) {
        
    }
}