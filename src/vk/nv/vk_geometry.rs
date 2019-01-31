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
use vk::nv::vk_geometry_type::*;
use vk::nv::vk_geometry_data::*;
use vk::nv::vk_geometry_flags::*;

#[derive(Debug, Clone)]
pub struct VkGeometry<'a, 'b, 'c, 'd> {
    pub geometry_type: VkGeometryType,
    pub geometry: VkGeometryData<'a, 'b, 'c, 'd>,
    pub flags: VkGeometryFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkGeometry {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub geometry_type: RawVkGeometryType,
    pub geometry: RawVkGeometryData,
    pub flags: RawVkGeometryFlags,
}

impl<'a, 'b, 'c, 'd> VkWrappedType<RawVkGeometry> for VkGeometry<'a, 'b, 'c, 'd> {
    fn vk_to_raw(src: &VkGeometry, dst: &mut RawVkGeometry) {
        dst.s_type = vk_to_raw_value(&VkStructureType::GeometryNv);
        dst.next = ptr::null();
        dst.geometry_type = vk_to_raw_value(&src.geometry_type);
        dst.geometry = vk_to_raw_value(&src.geometry);
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl Default for VkGeometry<'static, 'static, 'static, 'static> {
    fn default() -> VkGeometry<'static, 'static, 'static, 'static> {
        VkGeometry {
            geometry_type: VkGeometryType::default(),
            geometry: VkGeometryData::default(),
            flags: VkGeometryFlags::default(),
        }
    }
}

impl<'a, 'b, 'c, 'd> VkSetup for VkGeometry<'a, 'b, 'c, 'd> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkGeometry {
    fn vk_free(&mut self) {
        RawVkGeometryData::vk_free(&mut self.geometry);
    }
}