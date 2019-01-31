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
use vk::vk_memory_requirements::*;

#[derive(Debug, Clone)]
pub struct VkMemoryRequirements2 {
    pub memory_requirements: VkMemoryRequirements,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryRequirements2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub memory_requirements: RawVkMemoryRequirements,
}

impl VkWrappedType<RawVkMemoryRequirements2> for VkMemoryRequirements2 {
    fn vk_to_raw(src: &VkMemoryRequirements2, dst: &mut RawVkMemoryRequirements2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryRequirements2);
        dst.next = ptr::null();
        dst.memory_requirements = vk_to_raw_value(&src.memory_requirements);
    }
}

impl VkRawType<VkMemoryRequirements2> for RawVkMemoryRequirements2 {
    fn vk_to_wrapped(src: &RawVkMemoryRequirements2) -> VkMemoryRequirements2 {
        VkMemoryRequirements2 {
            memory_requirements: RawVkMemoryRequirements::vk_to_wrapped(&src.memory_requirements),
        }
    }
}

impl Default for VkMemoryRequirements2 {
    fn default() -> VkMemoryRequirements2 {
        VkMemoryRequirements2 {
            memory_requirements: VkMemoryRequirements::default(),
        }
    }
}

impl VkSetup for VkMemoryRequirements2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.memory_requirements, fn_table, instance, device);
    }
}

impl VkFree for RawVkMemoryRequirements2 {
    fn vk_free(&mut self) {
        RawVkMemoryRequirements::vk_free(&mut self.memory_requirements);
    }
}