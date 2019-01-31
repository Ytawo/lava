// Generated by `scripts/generate.js`

use utils::vk_traits::*;

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
    AccelerationStructureNv = 1000165000,
}

pub type RawVkDebugReportObjectType = i32;

impl VkWrappedType<RawVkDebugReportObjectType> for VkDebugReportObjectType {
    fn vk_to_raw(src: &VkDebugReportObjectType, dst: &mut RawVkDebugReportObjectType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkDebugReportObjectType> for RawVkDebugReportObjectType {
    fn vk_to_wrapped(src: &RawVkDebugReportObjectType) -> VkDebugReportObjectType {
        unsafe {
            *((src as *const i32) as *const VkDebugReportObjectType)
        }
    }
}

impl Default for VkDebugReportObjectType {
    fn default() -> VkDebugReportObjectType {
        VkDebugReportObjectType::Unknown
    }
}