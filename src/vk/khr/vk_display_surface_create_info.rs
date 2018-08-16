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
use vk::khr::vk_display_surface_create_flags::*;
use vk::khr::vk_display_mode::*;
use vk::khr::vk_surface_transform_flags::*;
use vk::khr::vk_display_plane_alpha_flags::*;
use vk::vk_extent_2d::*;

#[repr(C)]
pub struct RawVkDisplaySurfaceCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkDisplaySurfaceCreateFlags,
    display_mode: RawVkDisplayMode,
    plane_index: u32,
    plane_stack_index: u32,
    transform: RawVkSurfaceTransformFlags,
    global_alpha: f32,
    alpha_mode: RawVkDisplayPlaneAlphaFlags,
    image_extent: RawVkExtent2D,
}

#[derive(Debug, Clone)]
pub struct VkDisplaySurfaceCreateInfo<'a> {
    pub flags: VkDisplaySurfaceCreateFlags,
    pub display_mode: &'a VkDisplayMode,
    pub plane_index: usize,
    pub plane_stack_index: usize,
    pub transform: VkSurfaceTransformFlags,
    pub global_alpha: f32,
    pub alpha_mode: VkDisplayPlaneAlphaFlags,
    pub image_extent: VkExtent2D,
}

impl<'a> VkWrappedType<RawVkDisplaySurfaceCreateInfo> for VkDisplaySurfaceCreateInfo<'a> {
    fn vk_to_raw(src: &VkDisplaySurfaceCreateInfo, dst: &mut RawVkDisplaySurfaceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplaySurfaceCreateInfoKhr);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.display_mode = vk_to_raw_value(src.display_mode);
        dst.plane_index = vk_to_raw_value(&src.plane_index);
        dst.plane_stack_index = vk_to_raw_value(&src.plane_stack_index);
        dst.transform = vk_to_raw_value(&src.transform);
        dst.global_alpha = src.global_alpha;
        dst.alpha_mode = vk_to_raw_value(&src.alpha_mode);
        dst.image_extent = vk_to_raw_value(&src.image_extent);
    }
}

impl Default for VkDisplaySurfaceCreateInfo<'static> {
    fn default() -> VkDisplaySurfaceCreateInfo<'static> {
        VkDisplaySurfaceCreateInfo {
            flags: VkDisplaySurfaceCreateFlags::default(),
            display_mode: vk_null_ref(),
            plane_index: 0,
            plane_stack_index: 0,
            transform: VkSurfaceTransformFlags::default(),
            global_alpha: 0.0,
            alpha_mode: VkDisplayPlaneAlphaFlags::default(),
            image_extent: VkExtent2D::default(),
        }
    }
}

impl<'a> VkSetup for VkDisplaySurfaceCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.image_extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplaySurfaceCreateInfo {
    fn vk_free(&mut self) {
        RawVkExtent2D::vk_free(&mut self.image_extent);
    }
}