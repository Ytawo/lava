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
use vulkan::vk::{VkExternalMemoryHandleTypeFlags,RawVkExternalMemoryHandleTypeFlags};

/// Wrapper for [VkImportMemoryFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryFdInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkImportMemoryFdInfo {
    pub handle_type: VkExternalMemoryHandleTypeFlags,
    pub fd: i32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImportMemoryFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub handle_type: RawVkExternalMemoryHandleTypeFlags,
    pub fd: i32,
}

impl VkWrappedType<RawVkImportMemoryFdInfo> for VkImportMemoryFdInfo {
    fn vk_to_raw(src: &VkImportMemoryFdInfo, dst: &mut RawVkImportMemoryFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImportMemoryFdInfoKhr);
        dst.next = ptr::null_mut();
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
            handle_type: Default::default(),
            fd: 0,
        }
    }
}

impl VkSetup for VkImportMemoryFdInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkImportMemoryFdInfo {
    fn vk_free(&self) {
        
    }
}