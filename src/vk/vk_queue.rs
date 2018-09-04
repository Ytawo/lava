// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use vk::*;

pub type RawVkQueue = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkQueue {
    _handle: RawVkQueue,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkQueue> for RawVkQueue {
    fn vk_to_wrapped(src: &RawVkQueue) -> VkQueue {
        VkQueue {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkQueue> for VkQueue {
    fn vk_to_raw(src: &VkQueue, dst: &mut RawVkQueue) {
        *dst = src._handle
    }
}

impl Default for VkQueue {
    fn default() -> VkQueue {
        VkQueue {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkQueue {
    fn eq(&self, other: &VkQueue) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkQueue {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkQueue {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn submit(&self, submits: &[VkSubmitInfo], fence: Option<&VkFence>) -> Result<(), VkResult> {
        unsafe {
            let raw_submit_count = submits.len() as u32;
            let raw_submits = new_ptr_vk_array(submits);
            let raw_fence = if fence.is_some() { vk_to_raw_value(fence.unwrap()) } else { 0 };
            let vk_result = ((&*self._fn_table).vkQueueSubmit)(self._handle, raw_submit_count, raw_submits, raw_fence);
            free_vk_ptr_array(raw_submit_count as usize, raw_submits);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    pub fn wait_idle(&self) -> Result<(), VkResult> {
        unsafe {
            let vk_result = ((&*self._fn_table).vkQueueWaitIdle)(self._handle);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    pub fn bind_sparse(&self, bind_info: &[VkBindSparseInfo], fence: Option<&VkFence>) -> Result<(), VkResult> {
        unsafe {
            let raw_bind_info_count = bind_info.len() as u32;
            let raw_bind_info = new_ptr_vk_array(bind_info);
            let raw_fence = if fence.is_some() { vk_to_raw_value(fence.unwrap()) } else { 0 };
            let vk_result = ((&*self._fn_table).vkQueueBindSparse)(self._handle, raw_bind_info_count, raw_bind_info, raw_fence);
            free_vk_ptr_array(raw_bind_info_count as usize, raw_bind_info);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    pub fn present(&self, present_info: &khr::VkPresentInfo) -> Result<(), VkResult> {
        unsafe {
            let raw_present_info = new_ptr_vk_value(present_info);
            let vk_result = ((&*self._fn_table).vkQueuePresentKHR)(self._handle, raw_present_info);
            free_vk_ptr(raw_present_info);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    pub fn begin_debug_utils_label(&self, label_info: &ext::VkDebugUtilsLabel) {
        unsafe {
            let raw_label_info = new_ptr_vk_value(label_info);
            ((&*self._fn_table).vkQueueBeginDebugUtilsLabelEXT)(self._handle, raw_label_info);
            free_vk_ptr(raw_label_info);
        }
    }
    
    pub fn end_debug_utils_label(&self) {
        unsafe {
            ((&*self._fn_table).vkQueueEndDebugUtilsLabelEXT)(self._handle);
        }
    }
    
    pub fn insert_debug_utils_label(&self, label_info: &ext::VkDebugUtilsLabel) {
        unsafe {
            let raw_label_info = new_ptr_vk_value(label_info);
            ((&*self._fn_table).vkQueueInsertDebugUtilsLabelEXT)(self._handle, raw_label_info);
            free_vk_ptr(raw_label_info);
        }
    }
    
    pub fn get_checkpoint_data(&self) -> Vec<nv::VkCheckpointData> {
        unsafe {
            let mut raw_checkpoint_data : *mut nv::RawVkCheckpointData = ptr::null_mut();
            let raw_checkpoint_data_count = &mut mem::zeroed() as *mut u32;
            ((&*self._fn_table).vkGetQueueCheckpointDataNV)(self._handle, raw_checkpoint_data_count, raw_checkpoint_data);
            raw_checkpoint_data = calloc(*raw_checkpoint_data_count as usize, mem::size_of::<nv::RawVkCheckpointData>()) as *mut nv::RawVkCheckpointData;
            
            ((&*self._fn_table).vkGetQueueCheckpointDataNV)(self._handle, raw_checkpoint_data_count, raw_checkpoint_data);
            
            let mut checkpoint_data = new_vk_array(*raw_checkpoint_data_count, raw_checkpoint_data);
            for elt in &mut checkpoint_data { VkSetup::vk_setup(elt, self._fn_table, self._parent_instance, self._parent_device); }
            free_vk_ptr_array(*raw_checkpoint_data_count as usize, raw_checkpoint_data);
            checkpoint_data
        }
    }
}