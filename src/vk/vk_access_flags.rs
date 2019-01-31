// Generated by `scripts/generate.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkAccessFlags {
    pub indirect_command_read: bool,
    pub index_read: bool,
    pub vertex_attribute_read: bool,
    pub uniform_read: bool,
    pub input_attachment_read: bool,
    pub shader_read: bool,
    pub shader_write: bool,
    pub color_attachment_read: bool,
    pub color_attachment_write: bool,
    pub depth_stencil_attachment_read: bool,
    pub depth_stencil_attachment_write: bool,
    pub transfer_read: bool,
    pub transfer_write: bool,
    pub host_read: bool,
    pub host_write: bool,
    pub memory_read: bool,
    pub memory_write: bool,
    pub transform_feedback_write_ext: bool,
    pub transform_feedback_counter_read_ext: bool,
    pub transform_feedback_counter_write_ext: bool,
    pub conditional_rendering_read_ext: bool,
    pub command_process_read_nvx: bool,
    pub command_process_write_nvx: bool,
    pub color_attachment_read_noncoherent_ext: bool,
    pub shading_rate_image_read_nv: bool,
    pub acceleration_structure_read_nv: bool,
    pub acceleration_structure_write_nv: bool,
    pub fragment_density_map_read_ext: bool,
}

pub type RawVkAccessFlags = u32;

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
        if src.transform_feedback_write_ext { *dst |= 0x02000000; }
        if src.transform_feedback_counter_read_ext { *dst |= 0x04000000; }
        if src.transform_feedback_counter_write_ext { *dst |= 0x08000000; }
        if src.conditional_rendering_read_ext { *dst |= 0x00100000; }
        if src.command_process_read_nvx { *dst |= 0x00020000; }
        if src.command_process_write_nvx { *dst |= 0x00040000; }
        if src.color_attachment_read_noncoherent_ext { *dst |= 0x00080000; }
        if src.shading_rate_image_read_nv { *dst |= 0x00800000; }
        if src.acceleration_structure_read_nv { *dst |= 0x00200000; }
        if src.acceleration_structure_write_nv { *dst |= 0x00400000; }
        if src.fragment_density_map_read_ext { *dst |= 0x01000000; }
    }
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
            transform_feedback_write_ext: (src & 0x02000000) != 0,
            transform_feedback_counter_read_ext: (src & 0x04000000) != 0,
            transform_feedback_counter_write_ext: (src & 0x08000000) != 0,
            conditional_rendering_read_ext: (src & 0x00100000) != 0,
            command_process_read_nvx: (src & 0x00020000) != 0,
            command_process_write_nvx: (src & 0x00040000) != 0,
            color_attachment_read_noncoherent_ext: (src & 0x00080000) != 0,
            shading_rate_image_read_nv: (src & 0x00800000) != 0,
            acceleration_structure_read_nv: (src & 0x00200000) != 0,
            acceleration_structure_write_nv: (src & 0x00400000) != 0,
            fragment_density_map_read_ext: (src & 0x01000000) != 0,
        }
    }
}

impl Default for VkAccessFlags {
    fn default() -> VkAccessFlags {
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
            transform_feedback_write_ext: false,
            transform_feedback_counter_read_ext: false,
            transform_feedback_counter_write_ext: false,
            conditional_rendering_read_ext: false,
            command_process_read_nvx: false,
            command_process_write_nvx: false,
            color_attachment_read_noncoherent_ext: false,
            shading_rate_image_read_nv: false,
            acceleration_structure_read_nv: false,
            acceleration_structure_write_nv: false,
            fragment_density_map_read_ext: false,
        }
    }
}

impl VkAccessFlags {
    
    pub fn none() -> VkAccessFlags {
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
            transform_feedback_write_ext: false,
            transform_feedback_counter_read_ext: false,
            transform_feedback_counter_write_ext: false,
            conditional_rendering_read_ext: false,
            command_process_read_nvx: false,
            command_process_write_nvx: false,
            color_attachment_read_noncoherent_ext: false,
            shading_rate_image_read_nv: false,
            acceleration_structure_read_nv: false,
            acceleration_structure_write_nv: false,
            fragment_density_map_read_ext: false,
        }
    }
    
    pub fn all() -> VkAccessFlags {
        VkAccessFlags {
            indirect_command_read: true,
            index_read: true,
            vertex_attribute_read: true,
            uniform_read: true,
            input_attachment_read: true,
            shader_read: true,
            shader_write: true,
            color_attachment_read: true,
            color_attachment_write: true,
            depth_stencil_attachment_read: true,
            depth_stencil_attachment_write: true,
            transfer_read: true,
            transfer_write: true,
            host_read: true,
            host_write: true,
            memory_read: true,
            memory_write: true,
            transform_feedback_write_ext: true,
            transform_feedback_counter_read_ext: true,
            transform_feedback_counter_write_ext: true,
            conditional_rendering_read_ext: true,
            command_process_read_nvx: true,
            command_process_write_nvx: true,
            color_attachment_read_noncoherent_ext: true,
            shading_rate_image_read_nv: true,
            acceleration_structure_read_nv: true,
            acceleration_structure_write_nv: true,
            fragment_density_map_read_ext: true,
        }
    }
}

#[macro_export]
macro_rules! VkAccessFlags {
    ( $( $x:ident ),* ) => {
        VkAccessFlags {
            $($x: true,)*
            ..VkAccessFlags::none()
        }
    }
}

impl VkAccessFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.indirect_command_read { 0x00000001 } else { 0 }
        + if self.index_read { 0x00000002 } else { 0 }
        + if self.vertex_attribute_read { 0x00000004 } else { 0 }
        + if self.uniform_read { 0x00000008 } else { 0 }
        + if self.input_attachment_read { 0x00000010 } else { 0 }
        + if self.shader_read { 0x00000020 } else { 0 }
        + if self.shader_write { 0x00000040 } else { 0 }
        + if self.color_attachment_read { 0x00000080 } else { 0 }
        + if self.color_attachment_write { 0x00000100 } else { 0 }
        + if self.depth_stencil_attachment_read { 0x00000200 } else { 0 }
        + if self.depth_stencil_attachment_write { 0x00000400 } else { 0 }
        + if self.transfer_read { 0x00000800 } else { 0 }
        + if self.transfer_write { 0x00001000 } else { 0 }
        + if self.host_read { 0x00002000 } else { 0 }
        + if self.host_write { 0x00004000 } else { 0 }
        + if self.memory_read { 0x00008000 } else { 0 }
        + if self.memory_write { 0x00010000 } else { 0 }
        + if self.transform_feedback_write_ext { 0x02000000 } else { 0 }
        + if self.transform_feedback_counter_read_ext { 0x04000000 } else { 0 }
        + if self.transform_feedback_counter_write_ext { 0x08000000 } else { 0 }
        + if self.conditional_rendering_read_ext { 0x00100000 } else { 0 }
        + if self.command_process_read_nvx { 0x00020000 } else { 0 }
        + if self.command_process_write_nvx { 0x00040000 } else { 0 }
        + if self.color_attachment_read_noncoherent_ext { 0x00080000 } else { 0 }
        + if self.shading_rate_image_read_nv { 0x00800000 } else { 0 }
        + if self.acceleration_structure_read_nv { 0x00200000 } else { 0 }
        + if self.acceleration_structure_write_nv { 0x00400000 } else { 0 }
        + if self.fragment_density_map_read_ext { 0x01000000 } else { 0 }
    }
}