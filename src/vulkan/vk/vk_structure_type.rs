// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkStructureType](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkStructureType.html).
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
    PhysicalDeviceSubgroupProperties = 1000094000,
    BindBufferMemoryInfo = 1000157000,
    BindImageMemoryInfo = 1000157001,
    PhysicalDevice16bitStorageFeatures = 1000083000,
    MemoryDedicatedRequirements = 1000127000,
    MemoryDedicatedAllocateInfo = 1000127001,
    MemoryAllocateFlagsInfo = 1000060000,
    DeviceGroupRenderPassBeginInfo = 1000060003,
    DeviceGroupCommandBufferBeginInfo = 1000060004,
    DeviceGroupSubmitInfo = 1000060005,
    DeviceGroupBindSparseInfo = 1000060006,
    BindBufferMemoryDeviceGroupInfo = 1000060013,
    BindImageMemoryDeviceGroupInfo = 1000060014,
    PhysicalDeviceGroupProperties = 1000070000,
    DeviceGroupDeviceCreateInfo = 1000070001,
    BufferMemoryRequirementsInfo2 = 1000146000,
    ImageMemoryRequirementsInfo2 = 1000146001,
    ImageSparseMemoryRequirementsInfo2 = 1000146002,
    MemoryRequirements2 = 1000146003,
    SparseImageMemoryRequirements2 = 1000146004,
    PhysicalDeviceFeatures2 = 1000059000,
    PhysicalDeviceProperties2 = 1000059001,
    FormatProperties2 = 1000059002,
    ImageFormatProperties2 = 1000059003,
    PhysicalDeviceImageFormatInfo2 = 1000059004,
    QueueFamilyProperties2 = 1000059005,
    PhysicalDeviceMemoryProperties2 = 1000059006,
    SparseImageFormatProperties2 = 1000059007,
    PhysicalDeviceSparseImageFormatInfo2 = 1000059008,
    PhysicalDevicePointClippingProperties = 1000117000,
    RenderPassInputAttachmentAspectCreateInfo = 1000117001,
    ImageViewUsageCreateInfo = 1000117002,
    PipelineTessellationDomainOriginStateCreateInfo = 1000117003,
    RenderPassMultiviewCreateInfo = 1000053000,
    PhysicalDeviceMultiviewFeatures = 1000053001,
    PhysicalDeviceMultiviewProperties = 1000053002,
    PhysicalDeviceVariablePointerFeatures = 1000120000,
    ProtectedSubmitInfo = 1000145000,
    PhysicalDeviceProtectedMemoryFeatures = 1000145001,
    PhysicalDeviceProtectedMemoryProperties = 1000145002,
    DeviceQueueInfo2 = 1000145003,
    SamplerYcbcrConversionCreateInfo = 1000156000,
    SamplerYcbcrConversionInfo = 1000156001,
    BindImagePlaneMemoryInfo = 1000156002,
    ImagePlaneMemoryRequirementsInfo = 1000156003,
    PhysicalDeviceSamplerYcbcrConversionFeatures = 1000156004,
    SamplerYcbcrConversionImageFormatProperties = 1000156005,
    DescriptorUpdateTemplateCreateInfo = 1000085000,
    PhysicalDeviceExternalImageFormatInfo = 1000071000,
    ExternalImageFormatProperties = 1000071001,
    PhysicalDeviceExternalBufferInfo = 1000071002,
    ExternalBufferProperties = 1000071003,
    PhysicalDeviceIdProperties = 1000071004,
    ExternalMemoryBufferCreateInfo = 1000072000,
    ExternalMemoryImageCreateInfo = 1000072001,
    ExportMemoryAllocateInfo = 1000072002,
    PhysicalDeviceExternalFenceInfo = 1000112000,
    ExternalFenceProperties = 1000112001,
    ExportFenceCreateInfo = 1000113000,
    ExportSemaphoreCreateInfo = 1000077000,
    PhysicalDeviceExternalSemaphoreInfo = 1000076000,
    ExternalSemaphoreProperties = 1000076001,
    PhysicalDeviceMaintenance3Properties = 1000168000,
    DescriptorSetLayoutSupport = 1000168001,
    PhysicalDeviceShaderDrawParameterFeatures = 1000063000,
    SwapchainCreateInfoKhr = 1000001000,
    PresentInfoKhr = 1000001001,
    DeviceGroupPresentCapabilitiesKhr = 1000060007,
    ImageSwapchainCreateInfoKhr = 1000060008,
    BindImageMemorySwapchainInfoKhr = 1000060009,
    AcquireNextImageInfoKhr = 1000060010,
    DeviceGroupPresentInfoKhr = 1000060011,
    DeviceGroupSwapchainCreateInfoKhr = 1000060012,
    DisplayModeCreateInfoKhr = 1000002000,
    DisplaySurfaceCreateInfoKhr = 1000002001,
    DisplayPresentInfoKhr = 1000003000,
    XlibSurfaceCreateInfoKhr = 1000004000,
    XcbSurfaceCreateInfoKhr = 1000005000,
    WaylandSurfaceCreateInfoKhr = 1000006000,
    AndroidSurfaceCreateInfoKhr = 1000008000,
    Win32SurfaceCreateInfoKhr = 1000009000,
    DebugReportCallbackCreateInfoExt = 1000011000,
    PipelineRasterizationStateRasterizationOrderAmd = 1000018000,
    DebugMarkerObjectNameInfoExt = 1000022000,
    DebugMarkerObjectTagInfoExt = 1000022001,
    DebugMarkerMarkerInfoExt = 1000022002,
    DedicatedAllocationImageCreateInfoNv = 1000026000,
    DedicatedAllocationBufferCreateInfoNv = 1000026001,
    DedicatedAllocationMemoryAllocateInfoNv = 1000026002,
    PhysicalDeviceTransformFeedbackFeaturesExt = 1000028000,
    PhysicalDeviceTransformFeedbackPropertiesExt = 1000028001,
    PipelineRasterizationStateStreamCreateInfoExt = 1000028002,
    TextureLodGatherFormatPropertiesAmd = 1000041000,
    PhysicalDeviceCornerSampledImageFeaturesNv = 1000050000,
    ExternalMemoryImageCreateInfoNv = 1000056000,
    ExportMemoryAllocateInfoNv = 1000056001,
    ImportMemoryWin32HandleInfoNv = 1000057000,
    ExportMemoryWin32HandleInfoNv = 1000057001,
    Win32KeyedMutexAcquireReleaseInfoNv = 1000058000,
    ValidationFlagsExt = 1000061000,
    ViSurfaceCreateInfoNn = 1000062000,
    ImageViewAstcDecodeModeExt = 1000067000,
    PhysicalDeviceAstcDecodeFeaturesExt = 1000067001,
    ImportMemoryWin32HandleInfoKhr = 1000073000,
    ExportMemoryWin32HandleInfoKhr = 1000073001,
    MemoryWin32HandlePropertiesKhr = 1000073002,
    MemoryGetWin32HandleInfoKhr = 1000073003,
    ImportMemoryFdInfoKhr = 1000074000,
    MemoryFdPropertiesKhr = 1000074001,
    MemoryGetFdInfoKhr = 1000074002,
    Win32KeyedMutexAcquireReleaseInfoKhr = 1000075000,
    ImportSemaphoreWin32HandleInfoKhr = 1000078000,
    ExportSemaphoreWin32HandleInfoKhr = 1000078001,
    D3D12FenceSubmitInfoKhr = 1000078002,
    SemaphoreGetWin32HandleInfoKhr = 1000078003,
    ImportSemaphoreFdInfoKhr = 1000079000,
    SemaphoreGetFdInfoKhr = 1000079001,
    PhysicalDevicePushDescriptorPropertiesKhr = 1000080000,
    CommandBufferInheritanceConditionalRenderingInfoExt = 1000081000,
    PhysicalDeviceConditionalRenderingFeaturesExt = 1000081001,
    ConditionalRenderingBeginInfoExt = 1000081002,
    PhysicalDeviceFloat16Int8FeaturesKhr = 1000082000,
    PresentRegionsKhr = 1000084000,
    ObjectTableCreateInfoNvx = 1000086000,
    IndirectCommandsLayoutCreateInfoNvx = 1000086001,
    CmdProcessCommandsInfoNvx = 1000086002,
    CmdReserveSpaceForCommandsInfoNvx = 1000086003,
    DeviceGeneratedCommandsLimitsNvx = 1000086004,
    DeviceGeneratedCommandsFeaturesNvx = 1000086005,
    PipelineViewportWScalingStateCreateInfoNv = 1000087000,
    SurfaceCapabilities2Ext = 1000090000,
    DisplayPowerInfoExt = 1000091000,
    DeviceEventInfoExt = 1000091001,
    DisplayEventInfoExt = 1000091002,
    SwapchainCounterCreateInfoExt = 1000091003,
    PresentTimesInfoGoogle = 1000092000,
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx = 1000097000,
    PipelineViewportSwizzleStateCreateInfoNv = 1000098000,
    PhysicalDeviceDiscardRectanglePropertiesExt = 1000099000,
    PipelineDiscardRectangleStateCreateInfoExt = 1000099001,
    PhysicalDeviceConservativeRasterizationPropertiesExt = 1000101000,
    PipelineRasterizationConservativeStateCreateInfoExt = 1000101001,
    HdrMetadataExt = 1000105000,
    AttachmentDescription2Khr = 1000109000,
    AttachmentReference2Khr = 1000109001,
    SubpassDescription2Khr = 1000109002,
    SubpassDependency2Khr = 1000109003,
    RenderPassCreateInfo2Khr = 1000109004,
    SubpassBeginInfoKhr = 1000109005,
    SubpassEndInfoKhr = 1000109006,
    SharedPresentSurfaceCapabilitiesKhr = 1000111000,
    ImportFenceWin32HandleInfoKhr = 1000114000,
    ExportFenceWin32HandleInfoKhr = 1000114001,
    FenceGetWin32HandleInfoKhr = 1000114002,
    ImportFenceFdInfoKhr = 1000115000,
    FenceGetFdInfoKhr = 1000115001,
    PhysicalDeviceSurfaceInfo2Khr = 1000119000,
    SurfaceCapabilities2Khr = 1000119001,
    SurfaceFormat2Khr = 1000119002,
    DisplayProperties2Khr = 1000121000,
    DisplayPlaneProperties2Khr = 1000121001,
    DisplayModeProperties2Khr = 1000121002,
    DisplayPlaneInfo2Khr = 1000121003,
    DisplayPlaneCapabilities2Khr = 1000121004,
    IosSurfaceCreateInfoMvk = 1000122000,
    MacosSurfaceCreateInfoMvk = 1000123000,
    DebugUtilsObjectNameInfoExt = 1000128000,
    DebugUtilsObjectTagInfoExt = 1000128001,
    DebugUtilsLabelExt = 1000128002,
    DebugUtilsMessengerCallbackDataExt = 1000128003,
    DebugUtilsMessengerCreateInfoExt = 1000128004,
    AndroidHardwareBufferUsageAndroid = 1000129000,
    AndroidHardwareBufferPropertiesAndroid = 1000129001,
    AndroidHardwareBufferFormatPropertiesAndroid = 1000129002,
    ImportAndroidHardwareBufferInfoAndroid = 1000129003,
    MemoryGetAndroidHardwareBufferInfoAndroid = 1000129004,
    ExternalFormatAndroid = 1000129005,
    PhysicalDeviceSamplerFilterMinmaxPropertiesExt = 1000130000,
    SamplerReductionModeCreateInfoExt = 1000130001,
    PhysicalDeviceInlineUniformBlockFeaturesExt = 1000138000,
    PhysicalDeviceInlineUniformBlockPropertiesExt = 1000138001,
    WriteDescriptorSetInlineUniformBlockExt = 1000138002,
    DescriptorPoolInlineUniformBlockCreateInfoExt = 1000138003,
    SampleLocationsInfoExt = 1000143000,
    RenderPassSampleLocationsBeginInfoExt = 1000143001,
    PipelineSampleLocationsStateCreateInfoExt = 1000143002,
    PhysicalDeviceSampleLocationsPropertiesExt = 1000143003,
    MultisamplePropertiesExt = 1000143004,
    ImageFormatListCreateInfoKhr = 1000147000,
    PhysicalDeviceBlendOperationAdvancedFeaturesExt = 1000148000,
    PhysicalDeviceBlendOperationAdvancedPropertiesExt = 1000148001,
    PipelineColorBlendAdvancedStateCreateInfoExt = 1000148002,
    PipelineCoverageToColorStateCreateInfoNv = 1000149000,
    PipelineCoverageModulationStateCreateInfoNv = 1000152000,
    DrmFormatModifierPropertiesListExt = 1000158000,
    DrmFormatModifierPropertiesExt = 1000158001,
    PhysicalDeviceImageDrmFormatModifierInfoExt = 1000158002,
    ImageDrmFormatModifierListCreateInfoExt = 1000158003,
    ImageDrmFormatModifierExplicitCreateInfoExt = 1000158004,
    ImageDrmFormatModifierPropertiesExt = 1000158005,
    ValidationCacheCreateInfoExt = 1000160000,
    ShaderModuleValidationCacheCreateInfoExt = 1000160001,
    DescriptorSetLayoutBindingFlagsCreateInfoExt = 1000161000,
    PhysicalDeviceDescriptorIndexingFeaturesExt = 1000161001,
    PhysicalDeviceDescriptorIndexingPropertiesExt = 1000161002,
    DescriptorSetVariableDescriptorCountAllocateInfoExt = 1000161003,
    DescriptorSetVariableDescriptorCountLayoutSupportExt = 1000161004,
    PipelineViewportShadingRateImageStateCreateInfoNv = 1000164000,
    PhysicalDeviceShadingRateImageFeaturesNv = 1000164001,
    PhysicalDeviceShadingRateImagePropertiesNv = 1000164002,
    PipelineViewportCoarseSampleOrderStateCreateInfoNv = 1000164005,
    RayTracingPipelineCreateInfoNv = 1000165000,
    AccelerationStructureCreateInfoNv = 1000165001,
    GeometryNv = 1000165003,
    GeometryTrianglesNv = 1000165004,
    GeometryAabbNv = 1000165005,
    BindAccelerationStructureMemoryInfoNv = 1000165006,
    WriteDescriptorSetAccelerationStructureNv = 1000165007,
    AccelerationStructureMemoryRequirementsInfoNv = 1000165008,
    PhysicalDeviceRayTracingPropertiesNv = 1000165009,
    RayTracingShaderGroupCreateInfoNv = 1000165011,
    AccelerationStructureInfoNv = 1000165012,
    PhysicalDeviceRepresentativeFragmentTestFeaturesNv = 1000166000,
    PipelineRepresentativeFragmentTestStateCreateInfoNv = 1000166001,
    PhysicalDeviceImageViewImageFormatInfoExt = 1000170000,
    FilterCubicImageViewImageFormatPropertiesExt = 1000170001,
    DeviceQueueGlobalPriorityCreateInfoExt = 1000174000,
    PhysicalDevice8bitStorageFeaturesKhr = 1000177000,
    ImportMemoryHostPointerInfoExt = 1000178000,
    MemoryHostPointerPropertiesExt = 1000178001,
    PhysicalDeviceExternalMemoryHostPropertiesExt = 1000178002,
    PhysicalDeviceShaderAtomicInt64FeaturesKhr = 1000180000,
    CalibratedTimestampInfoExt = 1000184000,
    PhysicalDeviceShaderCorePropertiesAmd = 1000185000,
    DeviceMemoryOverallocationCreateInfoAmd = 1000189000,
    PhysicalDeviceVertexAttributeDivisorPropertiesExt = 1000190000,
    PipelineVertexInputDivisorStateCreateInfoExt = 1000190001,
    PhysicalDeviceVertexAttributeDivisorFeaturesExt = 1000190002,
    PhysicalDeviceDriverPropertiesKhr = 1000196000,
    PhysicalDeviceFloatControlsPropertiesKhr = 1000197000,
    PhysicalDeviceDepthStencilResolvePropertiesKhr = 1000199000,
    SubpassDescriptionDepthStencilResolveKhr = 1000199001,
    PhysicalDeviceComputeShaderDerivativesFeaturesNv = 1000201000,
    PhysicalDeviceMeshShaderFeaturesNv = 1000202000,
    PhysicalDeviceMeshShaderPropertiesNv = 1000202001,
    PhysicalDeviceFragmentShaderBarycentricFeaturesNv = 1000203000,
    PhysicalDeviceShaderImageFootprintFeaturesNv = 1000204000,
    PipelineViewportExclusiveScissorStateCreateInfoNv = 1000205000,
    PhysicalDeviceExclusiveScissorFeaturesNv = 1000205002,
    CheckpointDataNv = 1000206000,
    QueueFamilyCheckpointPropertiesNv = 1000206001,
    PhysicalDeviceVulkanMemoryModelFeaturesKhr = 1000211000,
    PhysicalDevicePciBusInfoPropertiesExt = 1000212000,
    ImagepipeSurfaceCreateInfoFuchsia = 1000214000,
    PhysicalDeviceFragmentDensityMapFeaturesExt = 1000218000,
    PhysicalDeviceFragmentDensityMapPropertiesExt = 1000218001,
    RenderPassFragmentDensityMapCreateInfoExt = 1000218002,
    PhysicalDeviceScalarBlockLayoutFeaturesExt = 1000221000,
    PhysicalDeviceMemoryBudgetPropertiesExt = 1000237000,
    PhysicalDeviceMemoryPriorityFeaturesExt = 1000238000,
    MemoryPriorityAllocateInfoExt = 1000238001,
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv = 1000240000,
    PhysicalDeviceBufferAddressFeaturesExt = 1000244000,
    BufferDeviceAddressInfoExt = 1000244001,
    BufferDeviceAddressCreateInfoExt = 1000244002,
    ImageStencilUsageCreateInfoExt = 1000246000,
    ValidationFeaturesExt = 1000247000,
}

#[doc(hidden)]
pub type RawVkStructureType = i32;

impl VkWrappedType<RawVkStructureType> for VkStructureType {
    fn vk_to_raw(src: &VkStructureType, dst: &mut RawVkStructureType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkStructureType> for RawVkStructureType {
    fn vk_to_wrapped(src: &RawVkStructureType) -> VkStructureType {
        unsafe {
            *((src as *const i32) as *const VkStructureType)
        }
    }
}

impl Default for VkStructureType {
    fn default() -> VkStructureType {
        VkStructureType::ApplicationInfo
    }
}