use std::os::raw::c_char;
use std::string::String;
use std::ffi::CString;
use std::*;
use utils::vk_type::*;
use utils::vk_convert::*;
use utils::vk_ptr::*;
use vk::vk_structure_type::*;
use vk::ext::vk_debug_report_flags::*;

pub type RawVkDebugCallback = unsafe extern fn(flags: u32, obj_type: i32, obj: u64, location: usize, code: i32, layer_prefix: *const c_char, msg: *const c_char, user_data: *mut c_void) -> u32;
pub type VkDebugCallback = fn(msg: String);

#[repr(C)]
pub struct RawVkDebugReportCallbackCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    callback: RawVkDebugCallback,
    user_data: *mut c_void
}

pub struct VkDebugReportCallbackCreateInfo {
    pub flags: VkDebugReportFlags,
    pub callback: VkDebugCallback
}

unsafe extern fn c_debug_callback(flags: u32, obj_type: i32, obj: u64, location: usize, code: i32, layer_prefix: *const c_char, msg: *const c_char, user_data: *mut c_void) -> u32 {
    let func : fn(String) = mem::transmute(user_data);
    let msg_str = new_string(msg);

    func(msg_str);

    0
}

impl VkWrappedType<RawVkDebugReportCallbackCreateInfo> for VkDebugReportCallbackCreateInfo {
    fn vk_to_raw(src: &VkDebugReportCallbackCreateInfo, dst: &mut RawVkDebugReportCallbackCreateInfo) {
        unsafe {
            dst.s_type = vk_to_raw_value(&VkStructureType::DebugReportCallbackCreateInfoExt);
            dst.p_next = ptr::null();
            dst.flags = vk_to_raw_value(&src.flags);
            dst.callback = c_debug_callback;
            dst.user_data = mem::transmute::<fn(String), *mut c_void>(src.callback);
        }
    }
}