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
use vk::vk_external_memory_handle_type_flags::*;

#[derive(Debug, Clone)]
pub struct VkImportMemoryHostPointerInfo {
    pub handle_type: VkExternalMemoryHandleTypeFlags,
    pub host_pointer: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImportMemoryHostPointerInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub handle_type: RawVkExternalMemoryHandleTypeFlags,
    pub host_pointer: *const c_void,
}

impl VkWrappedType<RawVkImportMemoryHostPointerInfo> for VkImportMemoryHostPointerInfo {
    fn vk_to_raw(src: &VkImportMemoryHostPointerInfo, dst: &mut RawVkImportMemoryHostPointerInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImportMemoryHostPointerInfoExt);
        dst.next = ptr::null();
        dst.handle_type = vk_to_raw_value(&src.handle_type);
        dst.host_pointer = src.host_pointer;
    }
}

impl VkRawType<VkImportMemoryHostPointerInfo> for RawVkImportMemoryHostPointerInfo {
    fn vk_to_wrapped(src: &RawVkImportMemoryHostPointerInfo) -> VkImportMemoryHostPointerInfo {
        VkImportMemoryHostPointerInfo {
            handle_type: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.handle_type),
            host_pointer: src.host_pointer,
        }
    }
}

impl Default for VkImportMemoryHostPointerInfo {
    fn default() -> VkImportMemoryHostPointerInfo {
        VkImportMemoryHostPointerInfo {
            handle_type: VkExternalMemoryHandleTypeFlags::default(),
            host_pointer: ptr::null(),
        }
    }
}

impl VkSetup for VkImportMemoryHostPointerInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImportMemoryHostPointerInfo {
    fn vk_free(&mut self) {
        
    }
}