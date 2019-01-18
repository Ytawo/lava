// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vk::*;

pub type RawVkCommandPool = u64;

#[derive(Debug, Clone)]
pub struct VkCommandPool {
    _handle: RawVkCommandPool,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkCommandPool> for RawVkCommandPool {
    fn vk_to_wrapped(src: &RawVkCommandPool) -> VkCommandPool {
        VkCommandPool {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkCommandPool> for VkCommandPool {
    fn vk_to_raw(src: &VkCommandPool, dst: &mut RawVkCommandPool) {
        *dst = src._handle
    }
}

impl Default for VkCommandPool {
    fn default() -> VkCommandPool {
        VkCommandPool {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkCommandPool {
    fn eq(&self, other: &VkCommandPool) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkCommandPool {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkCommandPool {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyCommandPool)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn reset(&self, flags: VkCommandPoolResetFlags) -> Result<(), VkResult> {
        unsafe {
            let raw_flags = vk_to_raw_value(&flags);
            let vk_result = ((&*self._fn_table).vkResetCommandPool)(self._parent_device, self._handle, raw_flags);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    pub fn free_command_buffers(&self, command_buffers: &[&VkCommandBuffer]) {
        unsafe {
            let raw_command_buffer_count = command_buffers.len() as u32;
            let raw_command_buffers = new_ptr_vk_array_from_ref(command_buffers);
            ((&*self._fn_table).vkFreeCommandBuffers)(self._parent_device, self._handle, raw_command_buffer_count, raw_command_buffers);
            free_ptr(raw_command_buffers);
        }
    }
    
    pub fn trim(&self, flags: VkCommandPoolTrimFlags) {
        unsafe {
            let raw_flags = vk_to_raw_value(&flags);
            ((&*self._fn_table).vkTrimCommandPool)(self._parent_device, self._handle, raw_flags);
        }
    }
}