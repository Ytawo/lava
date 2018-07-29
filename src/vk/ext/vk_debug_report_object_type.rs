// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDebugReportObjectType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDebugReportObjectType {
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,
    SurfaceKhr = 26,
    SwapchainKhr = 27,
    DebugReportCallbackExt = 28,
    DisplayKhr = 29,
    DisplayModeKhr = 30,
    ObjectTableNvx = 31,
    IndirectCommandsLayoutNvx = 32,
    ValidationCacheExt = 33,
    SamplerYcbcrConversion = 1000156000,
    DescriptorUpdateTemplate = 1000085000,
}

impl VkRawType<VkDebugReportObjectType> for RawVkDebugReportObjectType {
    fn vk_to_wrapped(src: &RawVkDebugReportObjectType) -> VkDebugReportObjectType {
        unsafe {
            *((src as *const i32) as *const VkDebugReportObjectType)
        }
    }
}

impl VkWrappedType<RawVkDebugReportObjectType> for VkDebugReportObjectType {
    fn vk_to_raw(src: &VkDebugReportObjectType, dst: &mut RawVkDebugReportObjectType) {
        *dst = *src as i32
    }
}

pub static STATIC_VK_DEBUG_REPORT_OBJECT_TYPE : VkDebugReportObjectType = VkDebugReportObjectType::Unknown;

impl VkDefault for VkDebugReportObjectType {
    fn vk_default() -> VkDebugReportObjectType {
        STATIC_VK_DEBUG_REPORT_OBJECT_TYPE
    }
}