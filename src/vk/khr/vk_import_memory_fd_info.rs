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
use vk::vk_external_memory_handle_type_flags::*;

#[derive(Debug, Clone)]
pub struct VkImportMemoryFdInfo {
    pub handle_type: VkExternalMemoryHandleTypeFlags,
    pub fd: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImportMemoryFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub handle_type: RawVkExternalMemoryHandleTypeFlags,
    pub fd: i32,
}

impl VkWrappedType<RawVkImportMemoryFdInfo> for VkImportMemoryFdInfo {
    fn vk_to_raw(src: &VkImportMemoryFdInfo, dst: &mut RawVkImportMemoryFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImportMemoryFdInfoKhr);
        dst.next = ptr::null();
        dst.handle_type = vk_to_raw_value(&src.handle_type);
        dst.fd = src.fd;
    }
}

impl VkRawType<VkImportMemoryFdInfo> for RawVkImportMemoryFdInfo {
    fn vk_to_wrapped(src: &RawVkImportMemoryFdInfo) -> VkImportMemoryFdInfo {
        VkImportMemoryFdInfo {
            handle_type: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.handle_type),
            fd: src.fd,
        }
    }
}

impl Default for VkImportMemoryFdInfo {
    fn default() -> VkImportMemoryFdInfo {
        VkImportMemoryFdInfo {
            handle_type: VkExternalMemoryHandleTypeFlags::default(),
            fd: 0,
        }
    }
}

impl VkSetup for VkImportMemoryFdInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImportMemoryFdInfo {
    fn vk_free(&mut self) {
        
    }
}