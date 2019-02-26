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
pub type RawVkSwapchain = u64;

/// Wrapper for [VkSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSwapchainKHR.html).
#[derive(Debug, Clone, Copy)]
pub struct VkSwapchain {
    _handle: RawVkSwapchain,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkSwapchain> for RawVkSwapchain {
    fn vk_to_wrapped(src: &RawVkSwapchain) -> VkSwapchain {
        VkSwapchain {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkSwapchain> for VkSwapchain {
    fn vk_to_raw(src: &VkSwapchain, dst: &mut RawVkSwapchain) {
        *dst = src._handle
    }
}

impl Default for VkSwapchain {
    fn default() -> VkSwapchain {
        VkSwapchain {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkSwapchain {
    fn eq(&self, other: &VkSwapchain) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkSwapchain {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkSwapchain {
    
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
    
    /// Wrapper for [vkDestroySwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroySwapchainKHR.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroySwapchainKHR)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetSwapchainImagesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetSwapchainImagesKHR.html).
    pub fn get_images(&self) -> LavaResult<Vec<VkImage>> {
        unsafe {
            let mut vk_result = 0;
            let mut raw_swapchain_images : *mut RawVkImage = ptr::null_mut();
            let raw_swapchain_image_count = &mut mem::zeroed() as *mut u32;
            vk_result = ((&*self._fn_table).vkGetSwapchainImagesKHR)((*self._fn_table).device, self._handle, raw_swapchain_image_count, raw_swapchain_images);
            raw_swapchain_images = calloc(*raw_swapchain_image_count as usize, mem::size_of::<RawVkImage>()) as *mut RawVkImage;
            
            vk_result = ((&*self._fn_table).vkGetSwapchainImagesKHR)((*self._fn_table).device, self._handle, raw_swapchain_image_count, raw_swapchain_images);
            
            let mut swapchain_images = new_vk_array(*raw_swapchain_image_count, raw_swapchain_images);
            if vk_result == 0 {
                for elt in &mut swapchain_images { VkSetup::vk_setup(elt, self._fn_table); }
            }
            free(raw_swapchain_images as *mut u8);
            if vk_result == 0 { Ok(swapchain_images) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), swapchain_images)) }
        }
    }
    
    /// Wrapper for [vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAcquireNextImageKHR.html).
    pub fn acquire_next_image(&self, timeout: u64, semaphore: Option<VkSemaphore>, fence: Option<VkFence>) -> LavaResult<usize> {
        unsafe {
            let raw_timeout = timeout;
            let raw_semaphore = vk_to_raw_value_checked(&semaphore);
            let raw_fence = vk_to_raw_value_checked(&fence);
            let mut vk_result = 0;
            let raw_image_index = &mut mem::zeroed() as *mut u32;
            
            vk_result = ((&*self._fn_table).vkAcquireNextImageKHR)((*self._fn_table).device, self._handle, raw_timeout, raw_semaphore, raw_fence, raw_image_index);
            
            let image_index = new_vk_value(raw_image_index);
            if vk_result == 0 { Ok(image_index) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), image_index)) }
        }
    }
    
    /// Wrapper for [vkGetSwapchainStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetSwapchainStatusKHR.html).
    pub fn get_status(&self) -> VkResult {
        unsafe {
            let vk_result = ((&*self._fn_table).vkGetSwapchainStatusKHR)((*self._fn_table).device, self._handle);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    /// Wrapper for [vkGetSwapchainCounterEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetSwapchainCounterEXT.html).
    pub fn get_counter(&self, counter: ext::VkSurfaceCounterFlags) -> LavaResult<usize> {
        unsafe {
            let raw_counter = vk_to_raw_value(&counter);
            let mut vk_result = 0;
            let raw_counter_value = &mut mem::zeroed() as *mut u64;
            
            vk_result = ((&*self._fn_table).vkGetSwapchainCounterEXT)((*self._fn_table).device, self._handle, raw_counter, raw_counter_value);
            
            let counter_value = new_vk_value(raw_counter_value);
            if vk_result == 0 { Ok(counter_value) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), counter_value)) }
        }
    }
    
    /// Wrapper for [vkGetRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html).
    pub fn get_refresh_cycle_duration(&self) -> LavaResult<google::VkRefreshCycleDuration> {
        unsafe {
            let mut vk_result = 0;
            let raw_display_timing_properties = &mut mem::zeroed() as *mut google::RawVkRefreshCycleDuration;
            
            vk_result = ((&*self._fn_table).vkGetRefreshCycleDurationGOOGLE)((*self._fn_table).device, self._handle, raw_display_timing_properties);
            
            let mut display_timing_properties = new_vk_value(raw_display_timing_properties);
            if vk_result == 0 {
                let fn_table = self._fn_table;
                VkSetup::vk_setup(&mut display_timing_properties, fn_table);
            }
            if vk_result == 0 { Ok(display_timing_properties) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), display_timing_properties)) }
        }
    }
    
    /// Wrapper for [vkGetPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html).
    pub fn get_past_presentation_timing(&self) -> LavaResult<Vec<google::VkPastPresentationTiming>> {
        unsafe {
            let mut vk_result = 0;
            let mut raw_presentation_timings : *mut google::RawVkPastPresentationTiming = ptr::null_mut();
            let raw_presentation_timing_count = &mut mem::zeroed() as *mut u32;
            vk_result = ((&*self._fn_table).vkGetPastPresentationTimingGOOGLE)((*self._fn_table).device, self._handle, raw_presentation_timing_count, raw_presentation_timings);
            raw_presentation_timings = calloc(*raw_presentation_timing_count as usize, mem::size_of::<google::RawVkPastPresentationTiming>()) as *mut google::RawVkPastPresentationTiming;
            
            vk_result = ((&*self._fn_table).vkGetPastPresentationTimingGOOGLE)((*self._fn_table).device, self._handle, raw_presentation_timing_count, raw_presentation_timings);
            
            let mut presentation_timings = new_vk_array(*raw_presentation_timing_count, raw_presentation_timings);
            if vk_result == 0 {
                for elt in &mut presentation_timings { VkSetup::vk_setup(elt, self._fn_table); }
            }
            free(raw_presentation_timings as *mut u8);
            if vk_result == 0 { Ok(presentation_timings) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), presentation_timings)) }
        }
    }
}