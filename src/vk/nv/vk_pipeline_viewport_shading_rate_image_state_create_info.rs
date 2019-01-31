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
use vk::nv::vk_shading_rate_palette::*;

#[derive(Debug, Clone)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfo<'a, 'b>
    where
        'b: 'a,
{
    pub shading_rate_image_enable: bool,
    pub shading_rate_palettes: Option<&'a [VkShadingRatePalette<'b>]>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineViewportShadingRateImageStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub shading_rate_image_enable: u32,
    pub viewport_count: u32,
    pub shading_rate_palettes: *mut RawVkShadingRatePalette,
}

impl<'a, 'b> VkWrappedType<RawVkPipelineViewportShadingRateImageStateCreateInfo> for VkPipelineViewportShadingRateImageStateCreateInfo<'a, 'b>
    where
        'b: 'a,
{
    fn vk_to_raw(src: &VkPipelineViewportShadingRateImageStateCreateInfo, dst: &mut RawVkPipelineViewportShadingRateImageStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineViewportShadingRateImageStateCreateInfoNv);
        dst.next = ptr::null();
        dst.shading_rate_image_enable = vk_to_raw_value(&src.shading_rate_image_enable);
        dst.viewport_count = get_array_option_len(src.shading_rate_palettes) as u32;
        dst.shading_rate_palettes = new_ptr_vk_array_checked(src.shading_rate_palettes);
    }
}

impl Default for VkPipelineViewportShadingRateImageStateCreateInfo<'static, 'static> {
    fn default() -> VkPipelineViewportShadingRateImageStateCreateInfo<'static, 'static> {
        VkPipelineViewportShadingRateImageStateCreateInfo {
            shading_rate_image_enable: false,
            shading_rate_palettes: None,
        }
    }
}

impl<'a, 'b> VkSetup for VkPipelineViewportShadingRateImageStateCreateInfo<'a, 'b>
    where
        'b: 'a,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineViewportShadingRateImageStateCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.viewport_count as usize, self.shading_rate_palettes);
    }
}