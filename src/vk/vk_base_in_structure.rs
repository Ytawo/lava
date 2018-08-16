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

#[repr(C)]
pub struct RawVkBaseInStructure {
    s_type: RawVkStructureType,
    next: *const c_void,
}

#[derive(Debug, Clone)]
pub struct VkBaseInStructure {
    pub s_type: VkStructureType,
}

impl VkRawType<VkBaseInStructure> for RawVkBaseInStructure {
    fn vk_to_wrapped(src: &RawVkBaseInStructure) -> VkBaseInStructure {
        VkBaseInStructure {
            s_type: RawVkStructureType::vk_to_wrapped(&src.s_type),
        }
    }
}

impl VkWrappedType<RawVkBaseInStructure> for VkBaseInStructure {
    fn vk_to_raw(src: &VkBaseInStructure, dst: &mut RawVkBaseInStructure) {
        dst.s_type = vk_to_raw_value(&src.s_type);
        dst.next = ptr::null();
    }
}

impl Default for VkBaseInStructure {
    fn default() -> VkBaseInStructure {
        VkBaseInStructure {
            s_type: VkStructureType::default(),
        }
    }
}

impl VkSetup for VkBaseInStructure {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBaseInStructure {
    fn vk_free(&mut self) {
        
    }
}