// Generated by `scripts/generate.js`

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
use vk::ext::vk_descriptor_binding_flags::*;

#[derive(Debug, Clone)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {
    pub binding_flags: Option<&'a [VkDescriptorBindingFlags]>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub binding_count: u32,
    pub binding_flags: *mut RawVkDescriptorBindingFlags,
}

impl<'a> VkWrappedType<RawVkDescriptorSetLayoutBindingFlagsCreateInfo> for VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {
    fn vk_to_raw(src: &VkDescriptorSetLayoutBindingFlagsCreateInfo, dst: &mut RawVkDescriptorSetLayoutBindingFlagsCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorSetLayoutBindingFlagsCreateInfoExt);
        dst.next = ptr::null();
        dst.binding_count = get_array_option_len(src.binding_flags) as u32;
        dst.binding_flags = new_ptr_vk_array_checked(src.binding_flags);
    }
}

impl Default for VkDescriptorSetLayoutBindingFlagsCreateInfo<'static> {
    fn default() -> VkDescriptorSetLayoutBindingFlagsCreateInfo<'static> {
        VkDescriptorSetLayoutBindingFlagsCreateInfo {
            binding_flags: None,
        }
    }
}

impl<'a> VkSetup for VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorSetLayoutBindingFlagsCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.binding_flags);
    }
}