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

pub type RawVkQueryPool = u64;

#[derive(Debug, Clone)]
pub struct VkQueryPool {
    _handle: RawVkQueryPool,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkQueryPool> for RawVkQueryPool {
    fn vk_to_wrapped(src: &RawVkQueryPool) -> VkQueryPool {
        VkQueryPool {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkQueryPool> for VkQueryPool {
    fn vk_to_raw(src: &VkQueryPool, dst: &mut RawVkQueryPool) {
        *dst = src._handle
    }
}

impl Default for VkQueryPool {
    fn default() -> VkQueryPool {
        VkQueryPool {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkQueryPool {
    fn eq(&self, other: &VkQueryPool) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkQueryPool {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkQueryPool {
    
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyQueryPool)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn get_results(&self, first_query: usize, query_count: usize, data: &mut [c_void], stride: usize, flags: VkQueryResultFlags) -> Result<(), VkResult> {
        unsafe {
            let raw_first_query = vk_to_raw_value(&first_query);
            let raw_query_count = vk_to_raw_value(&query_count);
            let raw_data_size = data.len();
            let raw_data = data.as_mut_ptr();
            let raw_stride = vk_to_raw_value(&stride);
            let raw_flags = vk_to_raw_value(&flags);
            let vk_result = ((&*self._fn_table).vkGetQueryPoolResults)(self._parent_device, self._handle, raw_first_query, raw_query_count, raw_data_size, raw_data, raw_stride, raw_flags);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
}