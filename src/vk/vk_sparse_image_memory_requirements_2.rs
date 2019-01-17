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
use vk::vk_sparse_image_memory_requirements::*;

#[derive(Debug, Clone)]
pub struct VkSparseImageMemoryRequirements2 {
    pub memory_requirements: VkSparseImageMemoryRequirements,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageMemoryRequirements2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub memory_requirements: RawVkSparseImageMemoryRequirements,
}

impl VkWrappedType<RawVkSparseImageMemoryRequirements2> for VkSparseImageMemoryRequirements2 {
    fn vk_to_raw(src: &VkSparseImageMemoryRequirements2, dst: &mut RawVkSparseImageMemoryRequirements2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SparseImageMemoryRequirements2);
        dst.next = ptr::null();
        dst.memory_requirements = vk_to_raw_value(&src.memory_requirements);
    }
}

impl VkRawType<VkSparseImageMemoryRequirements2> for RawVkSparseImageMemoryRequirements2 {
    fn vk_to_wrapped(src: &RawVkSparseImageMemoryRequirements2) -> VkSparseImageMemoryRequirements2 {
        VkSparseImageMemoryRequirements2 {
            memory_requirements: RawVkSparseImageMemoryRequirements::vk_to_wrapped(&src.memory_requirements),
        }
    }
}

impl Default for VkSparseImageMemoryRequirements2 {
    fn default() -> VkSparseImageMemoryRequirements2 {
        VkSparseImageMemoryRequirements2 {
            memory_requirements: VkSparseImageMemoryRequirements::default(),
        }
    }
}

impl VkSetup for VkSparseImageMemoryRequirements2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.memory_requirements, fn_table, instance, device);
    }
}

impl VkFree for RawVkSparseImageMemoryRequirements2 {
    fn vk_free(&mut self) {
        RawVkSparseImageMemoryRequirements::vk_free(&mut self.memory_requirements);
    }
}