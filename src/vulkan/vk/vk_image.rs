// Generated by `scripts/generate.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vulkan::*;
use vulkan::vk::*;

#[doc(hidden)]
pub type RawVkImage = u64;

/// Wrapper for [VkImage](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImage.html).
#[derive(Debug, Clone)]
pub struct VkImage {
    _handle: RawVkImage,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkImage> for RawVkImage {
    fn vk_to_wrapped(src: &RawVkImage) -> VkImage {
        VkImage {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkImage> for VkImage {
    fn vk_to_raw(src: &VkImage, dst: &mut RawVkImage) {
        *dst = src._handle
    }
}

impl Default for VkImage {
    fn default() -> VkImage {
        VkImage {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkImage {
    fn eq(&self, other: &VkImage) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkImage {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkImage {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Wrapper for [vkBindImageMemory](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkBindImageMemory.html).
    pub fn bind_memory(&self, memory: &VkDeviceMemory, memory_offset: usize) -> Result<(), VkResult> {
        unsafe {
            let raw_memory = vk_to_raw_value(memory);
            let raw_memory_offset = vk_to_raw_value(&memory_offset);
            let vk_result = ((&*self._fn_table).vkBindImageMemory)(self._parent_device, self._handle, raw_memory, raw_memory_offset);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkGetImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetImageMemoryRequirements.html).
    pub fn get_memory_requirements(&self) -> VkMemoryRequirements {
        unsafe {
            let raw_memory_requirements = &mut mem::zeroed() as *mut RawVkMemoryRequirements;
            
            ((&*self._fn_table).vkGetImageMemoryRequirements)(self._parent_device, self._handle, raw_memory_requirements);
            
            let mut memory_requirements = new_vk_value(raw_memory_requirements);
            let fn_table = self._fn_table;
            let parent_instance = self._parent_instance;
            let parent_device = self._parent_device;
            VkSetup::vk_setup(&mut memory_requirements, fn_table, parent_instance, parent_device);
            RawVkMemoryRequirements::vk_free(raw_memory_requirements.as_mut().unwrap());
            memory_requirements
        }
    }
    
    /// Wrapper for [vkGetImageSparseMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetImageSparseMemoryRequirements.html).
    pub fn get_sparse_memory_requirements(&self) -> Vec<VkSparseImageMemoryRequirements> {
        unsafe {
            let mut raw_sparse_memory_requirements : *mut RawVkSparseImageMemoryRequirements = ptr::null_mut();
            let raw_sparse_memory_requirement_count = &mut mem::zeroed() as *mut u32;
            ((&*self._fn_table).vkGetImageSparseMemoryRequirements)(self._parent_device, self._handle, raw_sparse_memory_requirement_count, raw_sparse_memory_requirements);
            raw_sparse_memory_requirements = calloc(*raw_sparse_memory_requirement_count as usize, mem::size_of::<RawVkSparseImageMemoryRequirements>()) as *mut RawVkSparseImageMemoryRequirements;
            
            ((&*self._fn_table).vkGetImageSparseMemoryRequirements)(self._parent_device, self._handle, raw_sparse_memory_requirement_count, raw_sparse_memory_requirements);
            
            let mut sparse_memory_requirements = new_vk_array(*raw_sparse_memory_requirement_count, raw_sparse_memory_requirements);
            for elt in &mut sparse_memory_requirements { VkSetup::vk_setup(elt, self._fn_table, self._parent_instance, self._parent_device); }
            free_vk_ptr_array(*raw_sparse_memory_requirement_count as usize, raw_sparse_memory_requirements);
            sparse_memory_requirements
        }
    }
    
    /// Wrapper for [vkDestroyImage](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyImage.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyImage)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetImageSubresourceLayout](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetImageSubresourceLayout.html).
    pub fn get_subresource_layout(&self, subresource: &VkImageSubresource) -> VkSubresourceLayout {
        unsafe {
            let raw_subresource = new_ptr_vk_value(subresource);
            let raw_layout = &mut mem::zeroed() as *mut RawVkSubresourceLayout;
            
            ((&*self._fn_table).vkGetImageSubresourceLayout)(self._parent_device, self._handle, raw_subresource, raw_layout);
            
            let mut layout = new_vk_value(raw_layout);
            let fn_table = self._fn_table;
            let parent_instance = self._parent_instance;
            let parent_device = self._parent_device;
            VkSetup::vk_setup(&mut layout, fn_table, parent_instance, parent_device);
            free_vk_ptr(raw_subresource);
            RawVkSubresourceLayout::vk_free(raw_layout.as_mut().unwrap());
            layout
        }
    }
    
    /// Wrapper for [vkGetImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html).
    pub fn get_drm_format_modifier_properties(&self) -> Result<ext::VkImageDrmFormatModifierProperties, (VkResult, ext::VkImageDrmFormatModifierProperties)> {
        unsafe {
            let mut vk_result = 0;
            let raw_properties = &mut mem::zeroed() as *mut ext::RawVkImageDrmFormatModifierProperties;
            
            vk_result = ((&*self._fn_table).vkGetImageDrmFormatModifierPropertiesEXT)(self._parent_device, self._handle, raw_properties);
            
            let mut properties = new_vk_value(raw_properties);
            if vk_result == 0 {
                let fn_table = self._fn_table;
                let parent_instance = self._parent_instance;
                let parent_device = self._parent_device;
                VkSetup::vk_setup(&mut properties, fn_table, parent_instance, parent_device);
            }
            ext::RawVkImageDrmFormatModifierProperties::vk_free(raw_properties.as_mut().unwrap());
            if vk_result == 0 { Ok(properties) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), properties)) }
        }
    }
}