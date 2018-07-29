// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDebugUtilsMessengerCallbackDataFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkDebugUtilsMessengerCallbackDataFlags {
    
}

impl VkRawType<VkDebugUtilsMessengerCallbackDataFlags> for RawVkDebugUtilsMessengerCallbackDataFlags {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessengerCallbackDataFlags) -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}

impl VkWrappedType<RawVkDebugUtilsMessengerCallbackDataFlags> for VkDebugUtilsMessengerCallbackDataFlags {
    fn vk_to_raw(src: &VkDebugUtilsMessengerCallbackDataFlags, dst: &mut RawVkDebugUtilsMessengerCallbackDataFlags) {
        *dst = 0;
    }
}

pub static STATIC_VK_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_FLAGS : VkDebugUtilsMessengerCallbackDataFlags = VkDebugUtilsMessengerCallbackDataFlags {
    
};

impl VkDefault for VkDebugUtilsMessengerCallbackDataFlags {
    fn vk_default() -> VkDebugUtilsMessengerCallbackDataFlags {
        STATIC_VK_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_FLAGS
    }
}