// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkAccessFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkAccessFlags {
    indirect_command_read: bool,
    index_read: bool,
    vertex_attribute_read: bool,
    uniform_read: bool,
    input_attachment_read: bool,
    shader_read: bool,
    shader_write: bool,
    color_attachment_read: bool,
    color_attachment_write: bool,
    depth_stencil_attachment_read: bool,
    depth_stencil_attachment_write: bool,
    transfer_read: bool,
    transfer_write: bool,
    host_read: bool,
    host_write: bool,
    memory_read: bool,
    memory_write: bool,
    command_process_read_nvx: bool,
    command_process_write_nvx: bool,
    color_attachment_read_noncoherent_ext: bool,
}

impl VkRawType<VkAccessFlags> for RawVkAccessFlags {
    
    fn vk_to_wrapped(src: &RawVkAccessFlags) -> VkAccessFlags {
        VkAccessFlags {
            indirect_command_read: (src & 0x00000001) != 0,
            index_read: (src & 0x00000002) != 0,
            vertex_attribute_read: (src & 0x00000004) != 0,
            uniform_read: (src & 0x00000008) != 0,
            input_attachment_read: (src & 0x00000010) != 0,
            shader_read: (src & 0x00000020) != 0,
            shader_write: (src & 0x00000040) != 0,
            color_attachment_read: (src & 0x00000080) != 0,
            color_attachment_write: (src & 0x00000100) != 0,
            depth_stencil_attachment_read: (src & 0x00000200) != 0,
            depth_stencil_attachment_write: (src & 0x00000400) != 0,
            transfer_read: (src & 0x00000800) != 0,
            transfer_write: (src & 0x00001000) != 0,
            host_read: (src & 0x00002000) != 0,
            host_write: (src & 0x00004000) != 0,
            memory_read: (src & 0x00008000) != 0,
            memory_write: (src & 0x00010000) != 0,
            command_process_read_nvx: (src & 0x00020000) != 0,
            command_process_write_nvx: (src & 0x00040000) != 0,
            color_attachment_read_noncoherent_ext: (src & 0x00080000) != 0,
        }
    }
}

impl VkWrappedType<RawVkAccessFlags> for VkAccessFlags {
    
    fn vk_to_raw(src: &VkAccessFlags, dst: &mut RawVkAccessFlags) {
        *dst = 0;
        if src.indirect_command_read { *dst |= 0x00000001; }
        if src.index_read { *dst |= 0x00000002; }
        if src.vertex_attribute_read { *dst |= 0x00000004; }
        if src.uniform_read { *dst |= 0x00000008; }
        if src.input_attachment_read { *dst |= 0x00000010; }
        if src.shader_read { *dst |= 0x00000020; }
        if src.shader_write { *dst |= 0x00000040; }
        if src.color_attachment_read { *dst |= 0x00000080; }
        if src.color_attachment_write { *dst |= 0x00000100; }
        if src.depth_stencil_attachment_read { *dst |= 0x00000200; }
        if src.depth_stencil_attachment_write { *dst |= 0x00000400; }
        if src.transfer_read { *dst |= 0x00000800; }
        if src.transfer_write { *dst |= 0x00001000; }
        if src.host_read { *dst |= 0x00002000; }
        if src.host_write { *dst |= 0x00004000; }
        if src.memory_read { *dst |= 0x00008000; }
        if src.memory_write { *dst |= 0x00010000; }
        if src.command_process_read_nvx { *dst |= 0x00020000; }
        if src.command_process_write_nvx { *dst |= 0x00040000; }
        if src.color_attachment_read_noncoherent_ext { *dst |= 0x00080000; }
    }
}

impl VkDefault for VkAccessFlags {
    
    fn vk_default() -> VkAccessFlags {
        VkAccessFlags {
            indirect_command_read: false,
            index_read: false,
            vertex_attribute_read: false,
            uniform_read: false,
            input_attachment_read: false,
            shader_read: false,
            shader_write: false,
            color_attachment_read: false,
            color_attachment_write: false,
            depth_stencil_attachment_read: false,
            depth_stencil_attachment_write: false,
            transfer_read: false,
            transfer_write: false,
            host_read: false,
            host_write: false,
            memory_read: false,
            memory_write: false,
            command_process_read_nvx: false,
            command_process_write_nvx: false,
            color_attachment_read_noncoherent_ext: false,
        }
    }
}