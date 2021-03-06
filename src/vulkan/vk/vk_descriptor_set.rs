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
pub type RawVkDescriptorSet = u64;

/// Wrapper for [VkDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSet.html).
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSet {
    _handle: RawVkDescriptorSet,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkDescriptorSet> for RawVkDescriptorSet {
    fn vk_to_wrapped(src: &RawVkDescriptorSet) -> VkDescriptorSet {
        VkDescriptorSet {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDescriptorSet> for VkDescriptorSet {
    fn vk_to_raw(src: &VkDescriptorSet, dst: &mut RawVkDescriptorSet) {
        *dst = src._handle
    }
}

impl Default for VkDescriptorSet {
    fn default() -> VkDescriptorSet {
        VkDescriptorSet {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDescriptorSet {
    fn eq(&self, other: &VkDescriptorSet) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkDescriptorSet {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkDescriptorSet {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Indicates if the Vulkan internal handle for this object is 0.
    pub fn is_null(&self) -> bool {
        self._handle == 0
    }
    
    /// Creates an object with a null Vulkan internal handle.
    ///
    /// Calling a method with a null handle will most likely result in a crash.
    pub fn null() -> Self {
        Self {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}