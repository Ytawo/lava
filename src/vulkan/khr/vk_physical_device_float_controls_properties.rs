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
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};

/// Wrapper for [VkPhysicalDeviceFloatControlsPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceFloatControlsPropertiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceFloatControlsProperties {
    pub separate_denorm_settings: bool,
    pub separate_rounding_mode_settings: bool,
    pub shader_signed_zero_inf_nan_preserve_float_16: bool,
    pub shader_signed_zero_inf_nan_preserve_float_32: bool,
    pub shader_signed_zero_inf_nan_preserve_float_64: bool,
    pub shader_denorm_preserve_float_16: bool,
    pub shader_denorm_preserve_float_32: bool,
    pub shader_denorm_preserve_float_64: bool,
    pub shader_denorm_flush_to_zero_float_16: bool,
    pub shader_denorm_flush_to_zero_float_32: bool,
    pub shader_denorm_flush_to_zero_float_64: bool,
    pub shader_rounding_mode_rtefloat_16: bool,
    pub shader_rounding_mode_rtefloat_32: bool,
    pub shader_rounding_mode_rtefloat_64: bool,
    pub shader_rounding_mode_rtzfloat_16: bool,
    pub shader_rounding_mode_rtzfloat_32: bool,
    pub shader_rounding_mode_rtzfloat_64: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceFloatControlsProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub separate_denorm_settings: u32,
    pub separate_rounding_mode_settings: u32,
    pub shader_signed_zero_inf_nan_preserve_float_16: u32,
    pub shader_signed_zero_inf_nan_preserve_float_32: u32,
    pub shader_signed_zero_inf_nan_preserve_float_64: u32,
    pub shader_denorm_preserve_float_16: u32,
    pub shader_denorm_preserve_float_32: u32,
    pub shader_denorm_preserve_float_64: u32,
    pub shader_denorm_flush_to_zero_float_16: u32,
    pub shader_denorm_flush_to_zero_float_32: u32,
    pub shader_denorm_flush_to_zero_float_64: u32,
    pub shader_rounding_mode_rtefloat_16: u32,
    pub shader_rounding_mode_rtefloat_32: u32,
    pub shader_rounding_mode_rtefloat_64: u32,
    pub shader_rounding_mode_rtzfloat_16: u32,
    pub shader_rounding_mode_rtzfloat_32: u32,
    pub shader_rounding_mode_rtzfloat_64: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceFloatControlsProperties> for VkPhysicalDeviceFloatControlsProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceFloatControlsProperties, dst: &mut RawVkPhysicalDeviceFloatControlsProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceFloatControlsPropertiesKhr);
        dst.next = ptr::null();
        dst.separate_denorm_settings = vk_to_raw_value(&src.separate_denorm_settings);
        dst.separate_rounding_mode_settings = vk_to_raw_value(&src.separate_rounding_mode_settings);
        dst.shader_signed_zero_inf_nan_preserve_float_16 = vk_to_raw_value(&src.shader_signed_zero_inf_nan_preserve_float_16);
        dst.shader_signed_zero_inf_nan_preserve_float_32 = vk_to_raw_value(&src.shader_signed_zero_inf_nan_preserve_float_32);
        dst.shader_signed_zero_inf_nan_preserve_float_64 = vk_to_raw_value(&src.shader_signed_zero_inf_nan_preserve_float_64);
        dst.shader_denorm_preserve_float_16 = vk_to_raw_value(&src.shader_denorm_preserve_float_16);
        dst.shader_denorm_preserve_float_32 = vk_to_raw_value(&src.shader_denorm_preserve_float_32);
        dst.shader_denorm_preserve_float_64 = vk_to_raw_value(&src.shader_denorm_preserve_float_64);
        dst.shader_denorm_flush_to_zero_float_16 = vk_to_raw_value(&src.shader_denorm_flush_to_zero_float_16);
        dst.shader_denorm_flush_to_zero_float_32 = vk_to_raw_value(&src.shader_denorm_flush_to_zero_float_32);
        dst.shader_denorm_flush_to_zero_float_64 = vk_to_raw_value(&src.shader_denorm_flush_to_zero_float_64);
        dst.shader_rounding_mode_rtefloat_16 = vk_to_raw_value(&src.shader_rounding_mode_rtefloat_16);
        dst.shader_rounding_mode_rtefloat_32 = vk_to_raw_value(&src.shader_rounding_mode_rtefloat_32);
        dst.shader_rounding_mode_rtefloat_64 = vk_to_raw_value(&src.shader_rounding_mode_rtefloat_64);
        dst.shader_rounding_mode_rtzfloat_16 = vk_to_raw_value(&src.shader_rounding_mode_rtzfloat_16);
        dst.shader_rounding_mode_rtzfloat_32 = vk_to_raw_value(&src.shader_rounding_mode_rtzfloat_32);
        dst.shader_rounding_mode_rtzfloat_64 = vk_to_raw_value(&src.shader_rounding_mode_rtzfloat_64);
    }
}

impl VkRawType<VkPhysicalDeviceFloatControlsProperties> for RawVkPhysicalDeviceFloatControlsProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceFloatControlsProperties) -> VkPhysicalDeviceFloatControlsProperties {
        VkPhysicalDeviceFloatControlsProperties {
            separate_denorm_settings: u32::vk_to_wrapped(&src.separate_denorm_settings),
            separate_rounding_mode_settings: u32::vk_to_wrapped(&src.separate_rounding_mode_settings),
            shader_signed_zero_inf_nan_preserve_float_16: u32::vk_to_wrapped(&src.shader_signed_zero_inf_nan_preserve_float_16),
            shader_signed_zero_inf_nan_preserve_float_32: u32::vk_to_wrapped(&src.shader_signed_zero_inf_nan_preserve_float_32),
            shader_signed_zero_inf_nan_preserve_float_64: u32::vk_to_wrapped(&src.shader_signed_zero_inf_nan_preserve_float_64),
            shader_denorm_preserve_float_16: u32::vk_to_wrapped(&src.shader_denorm_preserve_float_16),
            shader_denorm_preserve_float_32: u32::vk_to_wrapped(&src.shader_denorm_preserve_float_32),
            shader_denorm_preserve_float_64: u32::vk_to_wrapped(&src.shader_denorm_preserve_float_64),
            shader_denorm_flush_to_zero_float_16: u32::vk_to_wrapped(&src.shader_denorm_flush_to_zero_float_16),
            shader_denorm_flush_to_zero_float_32: u32::vk_to_wrapped(&src.shader_denorm_flush_to_zero_float_32),
            shader_denorm_flush_to_zero_float_64: u32::vk_to_wrapped(&src.shader_denorm_flush_to_zero_float_64),
            shader_rounding_mode_rtefloat_16: u32::vk_to_wrapped(&src.shader_rounding_mode_rtefloat_16),
            shader_rounding_mode_rtefloat_32: u32::vk_to_wrapped(&src.shader_rounding_mode_rtefloat_32),
            shader_rounding_mode_rtefloat_64: u32::vk_to_wrapped(&src.shader_rounding_mode_rtefloat_64),
            shader_rounding_mode_rtzfloat_16: u32::vk_to_wrapped(&src.shader_rounding_mode_rtzfloat_16),
            shader_rounding_mode_rtzfloat_32: u32::vk_to_wrapped(&src.shader_rounding_mode_rtzfloat_32),
            shader_rounding_mode_rtzfloat_64: u32::vk_to_wrapped(&src.shader_rounding_mode_rtzfloat_64),
        }
    }
}

impl Default for VkPhysicalDeviceFloatControlsProperties {
    fn default() -> VkPhysicalDeviceFloatControlsProperties {
        VkPhysicalDeviceFloatControlsProperties {
            separate_denorm_settings: false,
            separate_rounding_mode_settings: false,
            shader_signed_zero_inf_nan_preserve_float_16: false,
            shader_signed_zero_inf_nan_preserve_float_32: false,
            shader_signed_zero_inf_nan_preserve_float_64: false,
            shader_denorm_preserve_float_16: false,
            shader_denorm_preserve_float_32: false,
            shader_denorm_preserve_float_64: false,
            shader_denorm_flush_to_zero_float_16: false,
            shader_denorm_flush_to_zero_float_32: false,
            shader_denorm_flush_to_zero_float_64: false,
            shader_rounding_mode_rtefloat_16: false,
            shader_rounding_mode_rtefloat_32: false,
            shader_rounding_mode_rtefloat_64: false,
            shader_rounding_mode_rtzfloat_16: false,
            shader_rounding_mode_rtzfloat_32: false,
            shader_rounding_mode_rtzfloat_64: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceFloatControlsProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceFloatControlsProperties {
    fn vk_free(&self) {
        
    }
}