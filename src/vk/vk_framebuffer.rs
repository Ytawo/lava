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

pub type RawVkFramebuffer = u64;

#[derive(Debug, Clone)]
pub struct VkFramebuffer {
    _handle: RawVkFramebuffer,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkFramebuffer> for RawVkFramebuffer {
    fn vk_to_wrapped(src: &RawVkFramebuffer) -> VkFramebuffer {
        VkFramebuffer {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkFramebuffer> for VkFramebuffer {
    fn vk_to_raw(src: &VkFramebuffer, dst: &mut RawVkFramebuffer) {
        *dst = src._handle
    }
}

impl Default for VkFramebuffer {
    fn default() -> VkFramebuffer {
        VkFramebuffer {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkFramebuffer {
    fn eq(&self, other: &VkFramebuffer) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkFramebuffer {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkFramebuffer {
    
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyFramebuffer)(self._parent_device, self._handle, ptr::null());
        }
    }
}