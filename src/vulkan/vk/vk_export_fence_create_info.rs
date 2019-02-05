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
use vulkan::vk::{VkExternalFenceHandleTypeFlags,RawVkExternalFenceHandleTypeFlags};

/// Wrapper for [VkExportFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExportFenceCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkExportFenceCreateInfo {
    pub handle_types: VkExternalFenceHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExportFenceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub handle_types: RawVkExternalFenceHandleTypeFlags,
}

impl VkWrappedType<RawVkExportFenceCreateInfo> for VkExportFenceCreateInfo {
    fn vk_to_raw(src: &VkExportFenceCreateInfo, dst: &mut RawVkExportFenceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExportFenceCreateInfo);
        dst.next = ptr::null();
        dst.handle_types = vk_to_raw_value(&src.handle_types);
    }
}

impl VkRawType<VkExportFenceCreateInfo> for RawVkExportFenceCreateInfo {
    fn vk_to_wrapped(src: &RawVkExportFenceCreateInfo) -> VkExportFenceCreateInfo {
        VkExportFenceCreateInfo {
            handle_types: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.handle_types),
        }
    }
}

impl Default for VkExportFenceCreateInfo {
    fn default() -> VkExportFenceCreateInfo {
        VkExportFenceCreateInfo {
            handle_types: VkExternalFenceHandleTypeFlags::default(),
        }
    }
}

impl VkSetup for VkExportFenceCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkExportFenceCreateInfo {
    fn vk_free(&mut self) {
        
    }
}