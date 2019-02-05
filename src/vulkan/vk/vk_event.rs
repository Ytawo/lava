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
pub type RawVkEvent = u64;

/// Wrapper for [VkEvent](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkEvent.html).
#[derive(Debug, Clone, Copy)]
pub struct VkEvent {
    _handle: RawVkEvent,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkEvent> for RawVkEvent {
    fn vk_to_wrapped(src: &RawVkEvent) -> VkEvent {
        VkEvent {
            _handle: *src,
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
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkEvent {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Wrapper for [vkDestroyEvent](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyEvent.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyEvent)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetEventStatus](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetEventStatus.html).
    pub fn get_status(&self) -> VkResult {
        unsafe {
            let vk_result = ((&*self._fn_table).vkGetEventStatus)((*self._fn_table).device, self._handle);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    /// Wrapper for [vkSetEvent](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkSetEvent.html).
    pub fn set(&self) -> Result<(), VkResult> {
        unsafe {
            let vk_result = ((&*self._fn_table).vkSetEvent)((*self._fn_table).device, self._handle);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkResetEvent](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkResetEvent.html).
    pub fn reset(&self) -> Result<(), VkResult> {
        unsafe {
            let vk_result = ((&*self._fn_table).vkResetEvent)((*self._fn_table).device, self._handle);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
}