// Generated by `scripts/generate_vk.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::ext::vk_validation_cache_create_flags::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkValidationCacheCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkValidationCacheCreateFlags,
    pub initial_data_size: usize,
    pub initial_data: *const c_void,
}

#[derive(Debug, Clone)]
pub struct VkValidationCacheCreateInfo<'a> {
    pub flags: VkValidationCacheCreateFlags,
    pub initial_data_size: usize,
    pub initial_data: &'a c_void,
}

impl<'a> VkWrappedType<RawVkValidationCacheCreateInfo> for VkValidationCacheCreateInfo<'a> {
    fn vk_to_raw(src: &VkValidationCacheCreateInfo, dst: &mut RawVkValidationCacheCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ValidationCacheCreateInfoExt);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.initial_data_size = src.initial_data_size;
        dst.initial_data = src.initial_data as *const c_void;
    }
}

impl Default for VkValidationCacheCreateInfo<'static> {
    fn default() -> VkValidationCacheCreateInfo<'static> {
        VkValidationCacheCreateInfo {
            flags: VkValidationCacheCreateFlags::default(),
            initial_data_size: 0,
            initial_data: &0,
        }
    }
}

impl<'a> VkSetup for VkValidationCacheCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkValidationCacheCreateInfo {
    fn vk_free(&mut self) {
        
    }
}