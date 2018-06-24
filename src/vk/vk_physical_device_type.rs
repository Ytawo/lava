use std::convert::From;

pub type RawVkPhysicalDeviceType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4
}

impl<'a> From<&'a i32> for VkPhysicalDeviceType {
    fn from(value: &'a i32) -> Self {
        unsafe { *((value as *const i32) as *const VkPhysicalDeviceType) }
    }
}

impl<'a> From<&'a VkPhysicalDeviceType> for i32 {
    fn from(value: &'a VkPhysicalDeviceType) -> Self {
        *value as i32
    }
}