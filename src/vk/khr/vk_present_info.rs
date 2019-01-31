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
use vk::vk_semaphore::*;
use vk::khr::vk_swapchain::*;
use vk::vk_result::*;

#[derive(Debug, Clone)]
pub struct VkPresentInfo<'a, 'b, 'c, 'd, 'e, 'f>
    where
        'b: 'a,
        'd: 'c,
{
    pub wait_semaphores: &'a [&'b VkSemaphore],
    pub swapchains: &'c [&'d VkSwapchain],
    pub image_indices: &'e [usize],
    pub results: Option<&'f [VkResult]>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPresentInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *mut RawVkSemaphore,
    pub swapchain_count: u32,
    pub swapchains: *mut RawVkSwapchain,
    pub image_indices: *mut u32,
    pub results: *mut RawVkResult,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> VkWrappedType<RawVkPresentInfo> for VkPresentInfo<'a, 'b, 'c, 'd, 'e, 'f>
    where
        'b: 'a,
        'd: 'c,
{
    fn vk_to_raw(src: &VkPresentInfo, dst: &mut RawVkPresentInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PresentInfoKhr);
        dst.next = ptr::null();
        dst.wait_semaphore_count = src.wait_semaphores.len() as u32;
        dst.wait_semaphores = new_ptr_vk_array_from_ref(src.wait_semaphores);
        dst.swapchain_count = cmp::max(src.swapchains.len(), src.image_indices.len()) as u32;
        dst.swapchains = new_ptr_vk_array_from_ref(src.swapchains);
        dst.image_indices = new_ptr_vk_array(src.image_indices);
        dst.results = new_ptr_vk_array_checked(src.results);
    }
}

impl Default for VkPresentInfo<'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkPresentInfo<'static, 'static, 'static, 'static, 'static, 'static> {
        VkPresentInfo {
            wait_semaphores: &[],
            swapchains: &[],
            image_indices: &[],
            results: None,
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f> VkSetup for VkPresentInfo<'a, 'b, 'c, 'd, 'e, 'f>
    where
        'b: 'a,
        'd: 'c,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPresentInfo {
    fn vk_free(&mut self) {
        free_ptr(self.wait_semaphores);
        free_ptr(self.swapchains);
        free_ptr(self.image_indices);
        free_ptr(self.results);
    }
}