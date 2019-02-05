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
use vulkan::*;
use vulkan::vk::*;

#[doc(hidden)]
pub type RawVkQueryPool = u64;

/// Wrapper for [VkQueryPool](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPool.html).
#[derive(Debug, Clone, Copy)]
pub struct VkQueryPool {
    _handle: RawVkQueryPool,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkQueryPool> for RawVkQueryPool {
    fn vk_to_wrapped(src: &RawVkQueryPool) -> VkQueryPool {
        VkQueryPool {
            _handle: *src,
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
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkQueryPool {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Wrapper for [vkDestroyQueryPool](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyQueryPool.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyQueryPool)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetQueryPoolResults](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetQueryPoolResults.html).
    pub fn get_results(&self, first_query: usize, query_count: usize, data: &[c_void], stride: usize, flags: VkQueryResultFlags) -> Result<(), VkResult> {
        unsafe {
            let raw_first_query = vk_to_raw_value(&first_query);
            let raw_query_count = vk_to_raw_value(&query_count);
            let raw_data_size = data.len();
            let raw_data = data.as_ptr();
            let raw_stride = vk_to_raw_value(&stride);
            let raw_flags = vk_to_raw_value(&flags);
            let vk_result = ((&*self._fn_table).vkGetQueryPoolResults)((*self._fn_table).device, self._handle, raw_first_query, raw_query_count, raw_data_size, raw_data, raw_stride, raw_flags);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
}