// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use vk::*;

pub type RawVkPipeline = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkPipeline {
    _handle: RawVkPipeline,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkPipeline> for RawVkPipeline {
    fn vk_to_wrapped(src: &RawVkPipeline) -> VkPipeline {
        VkPipeline {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
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
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkSetup for VkPipeline {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkPipeline {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyPipeline)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn get_shader_info(&self, shader_stage: VkShaderStageFlags, info_type: amd::VkShaderInfoType) -> Result<Vec<c_void>, VkResult> {
        unsafe {
            let raw_shader_stage = vk_to_raw_value(&shader_stage);
            let raw_info_type = vk_to_raw_value(&info_type);
            let mut raw_info : *mut c_void = ptr::null_mut();
            let raw_info_size = &mut mem::uninitialized() as *mut usize;
            let vk_result = ((&*self._fn_table).vkGetShaderInfoAMD)(self._parent_device, self._handle, raw_shader_stage, raw_info_type, raw_info_size, raw_info);
            if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }
            raw_info = malloc((*raw_info_size as usize) * mem::size_of::<c_void>()) as *mut c_void;
            
            let vk_result = ((&*self._fn_table).vkGetShaderInfoAMD)(self._parent_device, self._handle, raw_shader_stage, raw_info_type, raw_info_size, raw_info);
            if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }
            
            let info = Vec::from_raw_parts(raw_info, *raw_info_size, *raw_info_size);
            Ok(info)
        }
    }
}