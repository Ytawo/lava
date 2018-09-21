// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vk::*;

pub type RawVkDescriptorSetLayout = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayout {
    _handle: RawVkDescriptorSetLayout,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkDescriptorSetLayout> for RawVkDescriptorSetLayout {
    fn vk_to_wrapped(src: &RawVkDescriptorSetLayout) -> VkDescriptorSetLayout {
        VkDescriptorSetLayout {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDescriptorSetLayout> for VkDescriptorSetLayout {
    fn vk_to_raw(src: &VkDescriptorSetLayout, dst: &mut RawVkDescriptorSetLayout) {
        *dst = src._handle
    }
}

impl Default for VkDescriptorSetLayout {
    fn default() -> VkDescriptorSetLayout {
        VkDescriptorSetLayout {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDescriptorSetLayout {
    fn eq(&self, other: &VkDescriptorSetLayout) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkDescriptorSetLayout {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkDescriptorSetLayout {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyDescriptorSetLayout)(self._parent_device, self._handle, ptr::null());
        }
    }
}