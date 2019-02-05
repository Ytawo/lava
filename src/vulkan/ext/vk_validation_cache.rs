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
pub type RawVkValidationCache = u64;

/// Wrapper for [VkValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkValidationCacheEXT.html).
#[derive(Debug, Clone)]
pub struct VkValidationCache {
    _handle: RawVkValidationCache,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkValidationCache> for RawVkValidationCache {
    fn vk_to_wrapped(src: &RawVkValidationCache) -> VkValidationCache {
        VkValidationCache {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkValidationCache> for VkValidationCache {
    fn vk_to_raw(src: &VkValidationCache, dst: &mut RawVkValidationCache) {
        *dst = src._handle
    }
}

impl Default for VkValidationCache {
    fn default() -> VkValidationCache {
        VkValidationCache {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkValidationCache {
    fn eq(&self, other: &VkValidationCache) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkValidationCache {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkValidationCache {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Wrapper for [vkDestroyValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyValidationCacheEXT.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyValidationCacheEXT)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkMergeValidationCachesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkMergeValidationCachesEXT.html).
    pub fn merge(&self, src_caches: &[&ext::VkValidationCache]) -> Result<(), VkResult> {
        unsafe {
            let raw_src_cache_count = src_caches.len() as u32;
            let raw_src_caches = new_ptr_vk_array_from_ref(src_caches);
            let vk_result = ((&*self._fn_table).vkMergeValidationCachesEXT)(self._parent_device, self._handle, raw_src_cache_count, raw_src_caches);
            free_ptr(raw_src_caches);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkGetValidationCacheDataEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetValidationCacheDataEXT.html).
    pub fn get_data(&self) -> Result<Vec<c_void>, (VkResult, Vec<c_void>)> {
        unsafe {
            let mut vk_result = 0;
            let mut raw_data : *mut c_void = ptr::null_mut();
            let raw_data_size = &mut mem::zeroed() as *mut usize;
            vk_result = ((&*self._fn_table).vkGetValidationCacheDataEXT)(self._parent_device, self._handle, raw_data_size, raw_data);
            raw_data = calloc(*raw_data_size as usize, mem::size_of::<c_void>()) as *mut c_void;
            
            vk_result = ((&*self._fn_table).vkGetValidationCacheDataEXT)(self._parent_device, self._handle, raw_data_size, raw_data);
            
            let data = Vec::from_raw_parts(raw_data, *raw_data_size, *raw_data_size);
            if vk_result == 0 { Ok(data) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), data)) }
        }
    }
}