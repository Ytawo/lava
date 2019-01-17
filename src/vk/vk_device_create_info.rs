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
use vk::vk_device_create_flags::*;
use vk::vk_device_queue_create_info::*;
use vk::vk_physical_device_features::*;

#[derive(Debug, Clone)]
pub struct VkDeviceCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'b: 'a,
        'd: 'c,
        'f: 'e,
{
    pub flags: VkDeviceCreateFlags,
    pub queue_create_infos: &'a [VkDeviceQueueCreateInfo<'b>],
    pub enabled_layer_names: &'c [&'d str],
    pub enabled_extension_names: &'e [&'f str],
    pub enabled_features: Option<&'g VkPhysicalDeviceFeatures>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub queue_create_infos: *mut RawVkDeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *mut *mut c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *mut *mut c_char,
    pub enabled_features: *mut RawVkPhysicalDeviceFeatures,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkWrappedType<RawVkDeviceCreateInfo> for VkDeviceCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'b: 'a,
        'd: 'c,
        'f: 'e,
{
    fn vk_to_raw(src: &VkDeviceCreateInfo, dst: &mut RawVkDeviceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.queue_create_info_count = src.queue_create_infos.len() as u32;
        dst.queue_create_infos = new_ptr_vk_array(src.queue_create_infos);
        dst.enabled_layer_count = src.enabled_layer_names.len() as u32;
        dst.enabled_layer_names = new_ptr_string_array(src.enabled_layer_names);
        dst.enabled_extension_count = src.enabled_extension_names.len() as u32;
        dst.enabled_extension_names = new_ptr_string_array(src.enabled_extension_names);
        dst.enabled_features = new_ptr_vk_value_checked(src.enabled_features);
    }
}

impl Default for VkDeviceCreateInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkDeviceCreateInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
        VkDeviceCreateInfo {
            flags: VkDeviceCreateFlags::default(),
            queue_create_infos: &[],
            enabled_layer_names: &[],
            enabled_extension_names: &[],
            enabled_features: None,
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkSetup for VkDeviceCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'b: 'a,
        'd: 'c,
        'f: 'e,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.queue_create_info_count as usize, self.queue_create_infos);
        free_ptr(self.enabled_layer_names);
        free_ptr(self.enabled_extension_names);
        free_vk_ptr(self.enabled_features);
    }
}