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

pub type RawVkFence = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkFence {
    _handle: RawVkFence,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkFence> for RawVkFence {
    fn vk_to_wrapped(src: &RawVkFence) -> VkFence {
        VkFence {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkFence> for VkFence {
    fn vk_to_raw(src: &VkFence, dst: &mut RawVkFence) {
        *dst = src._handle
    }
}

impl Default for VkFence {
    fn default() -> VkFence {
        VkFence {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkSetup for VkFence {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkFence {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyFence)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn get_status(&self) -> VkResult {
        unsafe {
            let vk_result = ((&*self._fn_table).vkGetFenceStatus)(self._parent_device, self._handle);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
}