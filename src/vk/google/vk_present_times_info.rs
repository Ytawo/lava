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
use vk::google::vk_present_time::*;

#[derive(Debug, Clone)]
pub struct VkPresentTimesInfo<'a> {
    pub times: Option<&'a [VkPresentTime]>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPresentTimesInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub swapchain_count: u32,
    pub times: *mut RawVkPresentTime,
}

impl<'a> VkWrappedType<RawVkPresentTimesInfo> for VkPresentTimesInfo<'a> {
    fn vk_to_raw(src: &VkPresentTimesInfo, dst: &mut RawVkPresentTimesInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PresentTimesInfoGoogle);
        dst.next = ptr::null();
        dst.swapchain_count = get_array_option_len(src.times) as u32;
        dst.times = new_ptr_vk_array_checked(src.times);
    }
}

impl Default for VkPresentTimesInfo<'static> {
    fn default() -> VkPresentTimesInfo<'static> {
        VkPresentTimesInfo {
            times: None,
        }
    }
}

impl<'a> VkSetup for VkPresentTimesInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPresentTimesInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.swapchain_count as usize, self.times);
    }
}