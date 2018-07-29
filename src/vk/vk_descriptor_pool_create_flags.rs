// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDescriptorPoolCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPoolCreateFlags {
    free_descriptor_set: bool,
    update_after_bind_ext: bool,
}

impl VkRawType<VkDescriptorPoolCreateFlags> for RawVkDescriptorPoolCreateFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorPoolCreateFlags) -> VkDescriptorPoolCreateFlags {
        VkDescriptorPoolCreateFlags {
            free_descriptor_set: (src & 0x00000001) != 0,
            update_after_bind_ext: (src & 0x00000002) != 0,
        }
    }
}

impl VkWrappedType<RawVkDescriptorPoolCreateFlags> for VkDescriptorPoolCreateFlags {
    fn vk_to_raw(src: &VkDescriptorPoolCreateFlags, dst: &mut RawVkDescriptorPoolCreateFlags) {
        *dst = 0;
        if src.free_descriptor_set { *dst |= 0x00000001; }
        if src.update_after_bind_ext { *dst |= 0x00000002; }
    }
}

pub static STATIC_VK_DESCRIPTOR_POOL_CREATE_FLAGS : VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlags {
    free_descriptor_set: false,
    update_after_bind_ext: false,
};

impl VkDefault for VkDescriptorPoolCreateFlags {
    fn vk_default() -> VkDescriptorPoolCreateFlags {
        STATIC_VK_DESCRIPTOR_POOL_CREATE_FLAGS
    }
}