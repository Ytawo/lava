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

pub type RawVkDisplayMode = u64;

#[derive(Debug, Clone)]
pub struct VkDisplayMode {
    _handle: RawVkDisplayMode,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkDisplayMode> for RawVkDisplayMode {
    fn vk_to_wrapped(src: &RawVkDisplayMode) -> VkDisplayMode {
        VkDisplayMode {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDisplayMode> for VkDisplayMode {
    fn vk_to_raw(src: &VkDisplayMode, dst: &mut RawVkDisplayMode) {
        *dst = src._handle
    }
}

impl Default for VkDisplayMode {
    fn default() -> VkDisplayMode {
        VkDisplayMode {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDisplayMode {
    fn eq(&self, other: &VkDisplayMode) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkDisplayMode {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkDisplayMode {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
}