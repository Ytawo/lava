// Generated by `scripts/generate_type.js`

use std::convert::From;
use std::default::Default;

pub type RawVkColorSpace = i32;



#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkColorSpace {
    SrgbNonlinear = 0
}

impl<'a> From<&'a i32> for VkColorSpace {
    fn from(value: &'a i32) -> Self {
        unsafe { *((value as *const i32) as *const VkColorSpace) }
    }
}

impl<'a> From<&'a VkColorSpace> for i32 {
    fn from(value: &'a VkColorSpace) -> Self {
        *value as i32
    }
}