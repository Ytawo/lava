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
use vk::vk_sampler::*;
use vk::vk_image_view::*;
use vk::vk_image_layout::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorImageInfo {
    pub sampler: RawVkSampler,
    pub image_view: RawVkImageView,
    pub image_layout: RawVkImageLayout,
}

#[derive(Debug, Clone)]
pub struct VkDescriptorImageInfo<'a, 'b> {
    pub sampler: &'a VkSampler,
    pub image_view: &'b VkImageView,
    pub image_layout: VkImageLayout,
}

impl<'a, 'b> VkWrappedType<RawVkDescriptorImageInfo> for VkDescriptorImageInfo<'a, 'b> {
    fn vk_to_raw(src: &VkDescriptorImageInfo, dst: &mut RawVkDescriptorImageInfo) {
        dst.sampler = vk_to_raw_value(src.sampler);
        dst.image_view = vk_to_raw_value(src.image_view);
        dst.image_layout = vk_to_raw_value(&src.image_layout);
    }
}

impl Default for VkDescriptorImageInfo<'static, 'static> {
    fn default() -> VkDescriptorImageInfo<'static, 'static> {
        VkDescriptorImageInfo {
            sampler: vk_null_ref(),
            image_view: vk_null_ref(),
            image_layout: VkImageLayout::default(),
        }
    }
}

impl<'a, 'b> VkSetup for VkDescriptorImageInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorImageInfo {
    fn vk_free(&mut self) {
        
    }
}