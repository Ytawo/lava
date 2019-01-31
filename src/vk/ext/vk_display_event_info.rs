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
use vk::ext::vk_display_event_type::*;

#[derive(Debug, Clone)]
pub struct VkDisplayEventInfo {
    pub display_event: VkDisplayEventType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayEventInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub display_event: RawVkDisplayEventType,
}

impl VkWrappedType<RawVkDisplayEventInfo> for VkDisplayEventInfo {
    fn vk_to_raw(src: &VkDisplayEventInfo, dst: &mut RawVkDisplayEventInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayEventInfoExt);
        dst.next = ptr::null();
        dst.display_event = vk_to_raw_value(&src.display_event);
    }
}

impl VkRawType<VkDisplayEventInfo> for RawVkDisplayEventInfo {
    fn vk_to_wrapped(src: &RawVkDisplayEventInfo) -> VkDisplayEventInfo {
        VkDisplayEventInfo {
            display_event: RawVkDisplayEventType::vk_to_wrapped(&src.display_event),
        }
    }
}

impl Default for VkDisplayEventInfo {
    fn default() -> VkDisplayEventInfo {
        VkDisplayEventInfo {
            display_event: VkDisplayEventType::default(),
        }
    }
}

impl VkSetup for VkDisplayEventInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDisplayEventInfo {
    fn vk_free(&mut self) {
        
    }
}