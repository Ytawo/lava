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
use vulkan::vk::*;
use vulkan::vk::{VkMemoryType,RawVkMemoryType};
use vulkan::vk::{VkMemoryHeap,RawVkMemoryHeap};

/// Wrapper for [VkPhysicalDeviceMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMemoryProperties.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memory_types: Vec<VkMemoryType>,
    pub memory_heaps: Vec<VkMemoryHeap>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [RawVkMemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [RawVkMemoryHeap; 16],
}

impl VkWrappedType<RawVkPhysicalDeviceMemoryProperties> for VkPhysicalDeviceMemoryProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceMemoryProperties, dst: &mut RawVkPhysicalDeviceMemoryProperties) {
        dst.memory_type_count = src.memory_types.len() as u32;
        dst.memory_types = unsafe { let mut dst_array : [RawVkMemoryType; 32] = mem::uninitialized(); vk_to_raw_array(&src.memory_types, &mut dst_array); dst_array };
        dst.memory_heap_count = src.memory_heaps.len() as u32;
        dst.memory_heaps = unsafe { let mut dst_array : [RawVkMemoryHeap; 16] = mem::uninitialized(); vk_to_raw_array(&src.memory_heaps, &mut dst_array); dst_array };
    }
}

impl VkRawType<VkPhysicalDeviceMemoryProperties> for RawVkPhysicalDeviceMemoryProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMemoryProperties) -> VkPhysicalDeviceMemoryProperties {
        VkPhysicalDeviceMemoryProperties {
            memory_types: new_vk_array(src.memory_type_count, src.memory_types.as_ptr()),
            memory_heaps: new_vk_array(src.memory_heap_count, src.memory_heaps.as_ptr()),
        }
    }
}

impl Default for VkPhysicalDeviceMemoryProperties {
    fn default() -> VkPhysicalDeviceMemoryProperties {
        VkPhysicalDeviceMemoryProperties {
            memory_types: Vec::new(),
            memory_heaps: Vec::new(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceMemoryProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMemoryProperties {
    fn vk_free(&mut self) {
        for elt in self.memory_types.iter_mut() { RawVkMemoryType::vk_free(elt); };
        for elt in self.memory_heaps.iter_mut() { RawVkMemoryHeap::vk_free(elt); };
    }
}