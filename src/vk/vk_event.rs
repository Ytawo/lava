// Generated by `scripts/generate.js`

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

pub type RawVkEvent = u64;

#[derive(Debug, Clone)]
pub struct VkEvent {
    _handle: RawVkEvent,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkEvent> for RawVkEvent {
    fn vk_to_wrapped(src: &RawVkEvent) -> VkEvent {
        VkEvent {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkEvent> for VkEvent {
    fn vk_to_raw(src: &VkEvent, dst: &mut RawVkEvent) {
        *dst = src._handle
    }
}

impl Default for VkEvent {
    fn default() -> VkEvent {
        VkEvent {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkEvent {
    fn eq(&self, other: &VkEvent) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkEvent {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkEvent {
    
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyEvent)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn get_status(&self) -> VkResult {
        unsafe {
            let vk_result = ((&*self._fn_table).vkGetEventStatus)(self._parent_device, self._handle);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    pub fn set(&self) -> Result<(), VkResult> {
        unsafe {
            let vk_result = ((&*self._fn_table).vkSetEvent)(self._parent_device, self._handle);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    pub fn reset(&self) -> Result<(), VkResult> {
        unsafe {
            let vk_result = ((&*self._fn_table).vkResetEvent)(self._parent_device, self._handle);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
}