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
use vk::khr::vk_swapchain_create_flags::*;
use vk::khr::vk_surface::*;
use vk::vk_format::*;
use vk::khr::vk_color_space::*;
use vk::vk_extent_2d::*;
use vk::vk_image_usage_flags::*;
use vk::vk_sharing_mode::*;
use vk::khr::vk_surface_transform_flags::*;
use vk::khr::vk_composite_alpha_flags::*;
use vk::khr::vk_present_mode::*;
use vk::khr::vk_swapchain::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSwapchainCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkSwapchainCreateFlags,
    pub surface: RawVkSurface,
    pub min_image_count: u32,
    pub image_format: RawVkFormat,
    pub image_color_space: RawVkColorSpace,
    pub image_extent: RawVkExtent2D,
    pub image_array_layers: u32,
    pub image_usage: RawVkImageUsageFlags,
    pub image_sharing_mode: RawVkSharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *mut u32,
    pub pre_transform: RawVkSurfaceTransformFlags,
    pub composite_alpha: RawVkCompositeAlphaFlags,
    pub present_mode: RawVkPresentMode,
    pub clipped: u32,
    pub old_swapchain: RawVkSwapchain,
}

#[derive(Debug, Clone)]
pub struct VkSwapchainCreateInfo<'a, 'b, 'c> {
    pub flags: VkSwapchainCreateFlags,
    pub surface: &'a VkSurface,
    pub min_image_count: usize,
    pub image_format: VkFormat,
    pub image_color_space: VkColorSpace,
    pub image_extent: VkExtent2D,
    pub image_array_layers: usize,
    pub image_usage: VkImageUsageFlags,
    pub image_sharing_mode: VkSharingMode,
    pub queue_family_indices: &'b [usize],
    pub pre_transform: VkSurfaceTransformFlags,
    pub composite_alpha: VkCompositeAlphaFlags,
    pub present_mode: VkPresentMode,
    pub clipped: bool,
    pub old_swapchain: Option<&'c VkSwapchain>,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkSwapchainCreateInfo> for VkSwapchainCreateInfo<'a, 'b, 'c> {
    fn vk_to_raw(src: &VkSwapchainCreateInfo, dst: &mut RawVkSwapchainCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SwapchainCreateInfoKhr);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.surface = vk_to_raw_value(src.surface);
        dst.min_image_count = vk_to_raw_value(&src.min_image_count);
        dst.image_format = vk_to_raw_value(&src.image_format);
        dst.image_color_space = vk_to_raw_value(&src.image_color_space);
        dst.image_extent = vk_to_raw_value(&src.image_extent);
        dst.image_array_layers = vk_to_raw_value(&src.image_array_layers);
        dst.image_usage = vk_to_raw_value(&src.image_usage);
        dst.image_sharing_mode = vk_to_raw_value(&src.image_sharing_mode);
        dst.queue_family_index_count = src.queue_family_indices.len() as u32;
        dst.queue_family_indices = new_ptr_vk_array(src.queue_family_indices);
        dst.pre_transform = vk_to_raw_value(&src.pre_transform);
        dst.composite_alpha = vk_to_raw_value(&src.composite_alpha);
        dst.present_mode = vk_to_raw_value(&src.present_mode);
        dst.clipped = vk_to_raw_value(&src.clipped);
        dst.old_swapchain = if src.old_swapchain.is_some() { vk_to_raw_value(src.old_swapchain.unwrap()) } else { 0 };
    }
}

impl Default for VkSwapchainCreateInfo<'static, 'static, 'static> {
    fn default() -> VkSwapchainCreateInfo<'static, 'static, 'static> {
        VkSwapchainCreateInfo {
            flags: VkSwapchainCreateFlags::default(),
            surface: vk_null_ref(),
            min_image_count: 0,
            image_format: VkFormat::default(),
            image_color_space: VkColorSpace::default(),
            image_extent: VkExtent2D::default(),
            image_array_layers: 0,
            image_usage: VkImageUsageFlags::default(),
            image_sharing_mode: VkSharingMode::default(),
            queue_family_indices: &[],
            pre_transform: VkSurfaceTransformFlags::default(),
            composite_alpha: VkCompositeAlphaFlags::default(),
            present_mode: VkPresentMode::default(),
            clipped: false,
            old_swapchain: None,
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkSwapchainCreateInfo<'a, 'b, 'c> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.image_extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkSwapchainCreateInfo {
    fn vk_free(&mut self) {
        RawVkExtent2D::vk_free(&mut self.image_extent);
        free_ptr(self.queue_family_indices);
    }
}