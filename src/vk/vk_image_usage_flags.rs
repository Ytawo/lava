// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkImageUsageFlags {
    pub transfer_src: bool,
    pub transfer_dst: bool,
    pub sampled: bool,
    pub storage: bool,
    pub color_attachment: bool,
    pub depth_stencil_attachment: bool,
    pub transient_attachment: bool,
    pub input_attachment: bool,
    pub shading_rate_image_nv: bool,
    pub fragment_density_map_ext: bool,
}

pub type RawVkImageUsageFlags = u32;

impl VkWrappedType<RawVkImageUsageFlags> for VkImageUsageFlags {
    fn vk_to_raw(src: &VkImageUsageFlags, dst: &mut RawVkImageUsageFlags) {
        *dst = 0;
        if src.transfer_src { *dst |= 0x00000001; }
        if src.transfer_dst { *dst |= 0x00000002; }
        if src.sampled { *dst |= 0x00000004; }
        if src.storage { *dst |= 0x00000008; }
        if src.color_attachment { *dst |= 0x00000010; }
        if src.depth_stencil_attachment { *dst |= 0x00000020; }
        if src.transient_attachment { *dst |= 0x00000040; }
        if src.input_attachment { *dst |= 0x00000080; }
        if src.shading_rate_image_nv { *dst |= 0x00000100; }
        if src.fragment_density_map_ext { *dst |= 0x00000200; }
    }
}

impl VkRawType<VkImageUsageFlags> for RawVkImageUsageFlags {
    fn vk_to_wrapped(src: &RawVkImageUsageFlags) -> VkImageUsageFlags {
        VkImageUsageFlags {
            transfer_src: (src & 0x00000001) != 0,
            transfer_dst: (src & 0x00000002) != 0,
            sampled: (src & 0x00000004) != 0,
            storage: (src & 0x00000008) != 0,
            color_attachment: (src & 0x00000010) != 0,
            depth_stencil_attachment: (src & 0x00000020) != 0,
            transient_attachment: (src & 0x00000040) != 0,
            input_attachment: (src & 0x00000080) != 0,
            shading_rate_image_nv: (src & 0x00000100) != 0,
            fragment_density_map_ext: (src & 0x00000200) != 0,
        }
    }
}

impl Default for VkImageUsageFlags {
    fn default() -> VkImageUsageFlags {
        VkImageUsageFlags {
            transfer_src: false,
            transfer_dst: false,
            sampled: false,
            storage: false,
            color_attachment: false,
            depth_stencil_attachment: false,
            transient_attachment: false,
            input_attachment: false,
            shading_rate_image_nv: false,
            fragment_density_map_ext: false,
        }
    }
}

impl VkImageUsageFlags {
    
    pub fn none() -> VkImageUsageFlags {
        VkImageUsageFlags {
            transfer_src: false,
            transfer_dst: false,
            sampled: false,
            storage: false,
            color_attachment: false,
            depth_stencil_attachment: false,
            transient_attachment: false,
            input_attachment: false,
            shading_rate_image_nv: false,
            fragment_density_map_ext: false,
        }
    }
    
    pub fn all() -> VkImageUsageFlags {
        VkImageUsageFlags {
            transfer_src: true,
            transfer_dst: true,
            sampled: true,
            storage: true,
            color_attachment: true,
            depth_stencil_attachment: true,
            transient_attachment: true,
            input_attachment: true,
            shading_rate_image_nv: true,
            fragment_density_map_ext: true,
        }
    }
}

impl VkImageUsageFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.transfer_src { 0x00000001 } else { 0 }
        + if self.transfer_dst { 0x00000002 } else { 0 }
        + if self.sampled { 0x00000004 } else { 0 }
        + if self.storage { 0x00000008 } else { 0 }
        + if self.color_attachment { 0x00000010 } else { 0 }
        + if self.depth_stencil_attachment { 0x00000020 } else { 0 }
        + if self.transient_attachment { 0x00000040 } else { 0 }
        + if self.input_attachment { 0x00000080 } else { 0 }
        + if self.shading_rate_image_nv { 0x00000100 } else { 0 }
        + if self.fragment_density_map_ext { 0x00000200 } else { 0 }
    }
}