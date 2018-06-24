use std::convert::From;

pub type RawVkStructureType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkStructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    LoaderInstanceCreateInfo = 47,
    LoaderDeviceCreateInfo = 48,
    SwapchainCreateInfoKhr = 1000001000,
    PresentInfoKhr = 1000001001,
    DisplayModeCreateInfoKhr = 1000002000,
    DisplaySurfaceCreateInfoKhr = 1000002001,
    DisplayPresentInfoKhr = 1000003000,
    XlibSurfaceCreateInfoKhr = 1000004000,
    XcbSurfaceCreateInfoKhr = 1000005000,
    WaylandSurfaceCreateInfoKhr = 1000006000,
    MirSurfaceCreateInfoKhr = 1000007000,
    AndroidSurfaceCreateInfoKhr = 1000008000,
    DebugReportCallbackCreateInfoExt = 1000011000,
    PipelineRasterizationStateRasterizationOrderAmd = 1000018000,
    DebugMarkerObjectNameInfoExt = 1000022000,
    DebugMarkerObjectTagInfoExt = 1000022001,
    DebugMarkerMarkerInfoExt = 1000022002,
    DedicatedAllocationImageCreateInfoNv = 1000026000,
    DedicatedAllocationBufferCreateInfoNv = 1000026001,
    DedicatedAllocationMemoryAllocateInfoNv = 1000026002,
    ExternalMemoryImageCreateInfoNv = 1000056000,
    ExportMemoryAllocateInfoNv = 1000056001,
    ValidationFlagsExt = 1000061000
}

impl<'a> From<&'a i32> for VkStructureType {
    fn from(value: &'a i32) -> Self {
        unsafe { *((value as *const i32) as *const VkStructureType) }
    }
}

impl<'a> From<&'a VkStructureType> for i32 {
    fn from(value: &'a VkStructureType) -> Self {
        *value as i32
    }
}