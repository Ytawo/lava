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

pub type RawVkDescriptorUpdateTemplate = u64;

#[derive(Debug, Clone)]
pub struct VkDescriptorUpdateTemplate {
    _handle: RawVkDescriptorUpdateTemplate,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkDescriptorUpdateTemplate> for RawVkDescriptorUpdateTemplate {
    fn vk_to_wrapped(src: &RawVkDescriptorUpdateTemplate) -> VkDescriptorUpdateTemplate {
        VkDescriptorUpdateTemplate {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDescriptorUpdateTemplate> for VkDescriptorUpdateTemplate {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplate, dst: &mut RawVkDescriptorUpdateTemplate) {
        *dst = src._handle
    }
}

impl Default for VkDescriptorUpdateTemplate {
    fn default() -> VkDescriptorUpdateTemplate {
        VkDescriptorUpdateTemplate {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDescriptorUpdateTemplate {
    fn eq(&self, other: &VkDescriptorUpdateTemplate) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkDescriptorUpdateTemplate {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkDescriptorUpdateTemplate {
    
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyDescriptorUpdateTemplate)(self._parent_device, self._handle, ptr::null());
        }
    }
}