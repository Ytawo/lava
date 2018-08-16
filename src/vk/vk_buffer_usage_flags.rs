// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkBufferUsageFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkBufferUsageFlags {
    pub transfer_src: bool,
    pub transfer_dst: bool,
    pub uniform_texel_buffer: bool,
    pub storage_texel_buffer: bool,
    pub uniform_buffer: bool,
    pub storage_buffer: bool,
    pub index_buffer: bool,
    pub vertex_buffer: bool,
    pub indirect_buffer: bool,
    pub conditional_rendering_ext: bool,
}

impl VkRawType<VkBufferUsageFlags> for RawVkBufferUsageFlags {
    fn vk_to_wrapped(src: &RawVkBufferUsageFlags) -> VkBufferUsageFlags {
        VkBufferUsageFlags {
            transfer_src: (src & 0x00000001) != 0,
            transfer_dst: (src & 0x00000002) != 0,
            uniform_texel_buffer: (src & 0x00000004) != 0,
            storage_texel_buffer: (src & 0x00000008) != 0,
            uniform_buffer: (src & 0x00000010) != 0,
            storage_buffer: (src & 0x00000020) != 0,
            index_buffer: (src & 0x00000040) != 0,
            vertex_buffer: (src & 0x00000080) != 0,
            indirect_buffer: (src & 0x00000100) != 0,
            conditional_rendering_ext: (src & 0x00000200) != 0,
        }
    }
}

impl VkWrappedType<RawVkBufferUsageFlags> for VkBufferUsageFlags {
    fn vk_to_raw(src: &VkBufferUsageFlags, dst: &mut RawVkBufferUsageFlags) {
        *dst = 0;
        if src.transfer_src { *dst |= 0x00000001; }
        if src.transfer_dst { *dst |= 0x00000002; }
        if src.uniform_texel_buffer { *dst |= 0x00000004; }
        if src.storage_texel_buffer { *dst |= 0x00000008; }
        if src.uniform_buffer { *dst |= 0x00000010; }
        if src.storage_buffer { *dst |= 0x00000020; }
        if src.index_buffer { *dst |= 0x00000040; }
        if src.vertex_buffer { *dst |= 0x00000080; }
        if src.indirect_buffer { *dst |= 0x00000100; }
        if src.conditional_rendering_ext { *dst |= 0x00000200; }
    }
}

impl Default for VkBufferUsageFlags {
    fn default() -> VkBufferUsageFlags {
        VkBufferUsageFlags {
            transfer_src: false,
            transfer_dst: false,
            uniform_texel_buffer: false,
            storage_texel_buffer: false,
            uniform_buffer: false,
            storage_buffer: false,
            index_buffer: false,
            vertex_buffer: false,
            indirect_buffer: false,
            conditional_rendering_ext: false,
        }
    }
}

impl VkBufferUsageFlags {
    
    pub fn none() -> VkBufferUsageFlags {
        VkBufferUsageFlags {
            transfer_src: false,
            transfer_dst: false,
            uniform_texel_buffer: false,
            storage_texel_buffer: false,
            uniform_buffer: false,
            storage_buffer: false,
            index_buffer: false,
            vertex_buffer: false,
            indirect_buffer: false,
            conditional_rendering_ext: false,
        }
    }
    
    pub fn all() -> VkBufferUsageFlags {
        VkBufferUsageFlags {
            transfer_src: true,
            transfer_dst: true,
            uniform_texel_buffer: true,
            storage_texel_buffer: true,
            uniform_buffer: true,
            storage_buffer: true,
            index_buffer: true,
            vertex_buffer: true,
            indirect_buffer: true,
            conditional_rendering_ext: true,
        }
    }
}