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
use vk::vk_buffer::*;

#[derive(Debug, Clone)]
pub struct VkGeometryAABB<'a> {
    pub aabb_data: Option<&'a VkBuffer>,
    pub num_aabbs: usize,
    pub stride: usize,
    pub offset: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkGeometryAABB {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub aabb_data: RawVkBuffer,
    pub num_aabbs: u32,
    pub stride: u32,
    pub offset: u64,
}

impl<'a> VkWrappedType<RawVkGeometryAABB> for VkGeometryAABB<'a> {
    fn vk_to_raw(src: &VkGeometryAABB, dst: &mut RawVkGeometryAABB) {
        dst.s_type = vk_to_raw_value(&VkStructureType::GeometryAabbNv);
        dst.next = ptr::null();
        dst.aabb_data = if src.aabb_data.is_some() { vk_to_raw_value(src.aabb_data.unwrap()) } else { 0 };
        dst.num_aabbs = vk_to_raw_value(&src.num_aabbs);
        dst.stride = vk_to_raw_value(&src.stride);
        dst.offset = vk_to_raw_value(&src.offset);
    }
}

impl Default for VkGeometryAABB<'static> {
    fn default() -> VkGeometryAABB<'static> {
        VkGeometryAABB {
            aabb_data: None,
            num_aabbs: 0,
            stride: 0,
            offset: 0,
        }
    }
}

impl<'a> VkSetup for VkGeometryAABB<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkGeometryAABB {
    fn vk_free(&mut self) {
        
    }
}