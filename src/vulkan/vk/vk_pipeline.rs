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
pub type RawVkPipeline = u64;

/// Wrapper for [VkPipeline](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipeline.html).
#[derive(Debug, Clone, Copy)]
pub struct VkPipeline {
    _handle: RawVkPipeline,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkPipeline> for RawVkPipeline {
    fn vk_to_wrapped(src: &RawVkPipeline) -> VkPipeline {
        VkPipeline {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkPipeline> for VkPipeline {
    fn vk_to_raw(src: &VkPipeline, dst: &mut RawVkPipeline) {
        *dst = src._handle
    }
}

impl Default for VkPipeline {
    fn default() -> VkPipeline {
        VkPipeline {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkPipeline {
    fn eq(&self, other: &VkPipeline) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkPipeline {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkPipeline {
    
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
    
    /// Wrapper for [vkDestroyPipeline](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipeline.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyPipeline)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetShaderInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetShaderInfoAMD.html).
    pub fn get_shader_info(&self, shader_stage: VkShaderStageFlags, info_type: amd::VkShaderInfoType) -> LavaResult<Vec<c_void>> {
        unsafe {
            let raw_shader_stage = vk_to_raw_value(&shader_stage);
            let raw_info_type = vk_to_raw_value(&info_type);
            let mut vk_result = 0;
            let mut raw_info : *mut c_void = ptr::null_mut();
            let raw_info_size = &mut mem::zeroed() as *mut usize;
            vk_result = ((&*self._fn_table).vkGetShaderInfoAMD)((*self._fn_table).device, self._handle, raw_shader_stage, raw_info_type, raw_info_size, raw_info);
            raw_info = calloc(*raw_info_size as usize, mem::size_of::<c_void>()) as *mut c_void;
            
            vk_result = ((&*self._fn_table).vkGetShaderInfoAMD)((*self._fn_table).device, self._handle, raw_shader_stage, raw_info_type, raw_info_size, raw_info);
            
            let info = Vec::from_raw_parts(raw_info, *raw_info_size, *raw_info_size);
            if vk_result == 0 { Ok(info) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), info)) }
        }
    }
    
    /// Wrapper for [vkGetRayTracingShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html).
    pub fn get_ray_tracing_shader_group_handles(&self, first_group: usize, group_count: usize, data: &[c_void]) -> LavaResult<()> {
        unsafe {
            let raw_first_group = vk_to_raw_value(&first_group);
            let raw_group_count = vk_to_raw_value(&group_count);
            let raw_data_size = data.len();
            let raw_data = get_vec_ptr(data);
            let vk_result = ((&*self._fn_table).vkGetRayTracingShaderGroupHandlesKHR)((*self._fn_table).device, self._handle, raw_first_group, raw_group_count, raw_data_size, raw_data);
            if vk_result == 0 { Ok(()) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), ())) }
        }
    }
    
    /// Wrapper for [vkCompileDeferredNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html).
    pub fn compile_deferred(&self, shader: usize) -> LavaResult<()> {
        unsafe {
            let raw_shader = vk_to_raw_value(&shader);
            let vk_result = ((&*self._fn_table).vkCompileDeferredNV)((*self._fn_table).device, self._handle, raw_shader);
            if vk_result == 0 { Ok(()) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), ())) }
        }
    }
}