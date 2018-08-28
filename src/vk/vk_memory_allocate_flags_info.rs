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
use vk::vk_memory_allocate_flags::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryAllocateFlagsInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkMemoryAllocateFlags,
    pub device_mask: u32,
}

#[derive(Debug, Clone)]
pub struct VkMemoryAllocateFlagsInfo {
    pub flags: VkMemoryAllocateFlags,
    pub device_mask: u32,
}

impl VkRawType<VkMemoryAllocateFlagsInfo> for RawVkMemoryAllocateFlagsInfo {
    fn vk_to_wrapped(src: &RawVkMemoryAllocateFlagsInfo) -> VkMemoryAllocateFlagsInfo {
        VkMemoryAllocateFlagsInfo {
            flags: RawVkMemoryAllocateFlags::vk_to_wrapped(&src.flags),
            device_mask: src.device_mask,
        }
    }
}

impl VkWrappedType<RawVkMemoryAllocateFlagsInfo> for VkMemoryAllocateFlagsInfo {
    fn vk_to_raw(src: &VkMemoryAllocateFlagsInfo, dst: &mut RawVkMemoryAllocateFlagsInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryAllocateFlagsInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.device_mask = src.device_mask;
    }
}

impl Default for VkMemoryAllocateFlagsInfo {
    fn default() -> VkMemoryAllocateFlagsInfo {
        VkMemoryAllocateFlagsInfo {
            flags: VkMemoryAllocateFlags::default(),
            device_mask: 0,
        }
    }
}

impl VkSetup for VkMemoryAllocateFlagsInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMemoryAllocateFlagsInfo {
    fn vk_free(&mut self) {
        
    }
}