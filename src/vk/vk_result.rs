use std::convert::From;

pub type RawVkResult = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorSurfaceLostKhr = -1000000000,
    ErrorNativeWindowInUseKhr = -1000000001,
    SuboptimalKhr = 1000001003,
    ErrorOutOfDateKhr = -1000001004,
    ErrorIncompatibleDisplayKhr = -1000003001,
    ErrorValidationFailedExt = -1000011001,
    ErrorInvalidShaderNv = -1000012000
}

impl<'a> From<&'a i32> for VkResult {
    fn from(value: &'a i32) -> Self {
        unsafe { *((value as *const i32) as *const VkResult) }
    }
}

impl<'a> From<&'a VkResult> for i32 {
    fn from(value: &'a VkResult) -> Self {
        *value as i32
    }
}