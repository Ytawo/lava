// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkFormatFeatureFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFormatFeatureFlags.html).
///
/// Use the macro `VkFormatFeatureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkFormatFeatureFlags!(sampled_image, storage_image)
/// ```
/// ```
/// VkFormatFeatureFlags {
///     sampled_image: true,
///     storage_image: true,
///     ..VkFormatFeatureFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkFormatFeatureFlags {
    pub sampled_image: bool,
    pub storage_image: bool,
    pub storage_image_atomic: bool,
    pub uniform_texel_buffer: bool,
    pub storage_texel_buffer: bool,
    pub storage_texel_buffer_atomic: bool,
    pub vertex_buffer: bool,
    pub color_attachment: bool,
    pub color_attachment_blend: bool,
    pub depth_stencil_attachment: bool,
    pub blit_src: bool,
    pub blit_dst: bool,
    pub sampled_image_filter_linear: bool,
    pub transfer_src: bool,
    pub transfer_dst: bool,
    pub midpoint_chroma_samples: bool,
    pub sampled_image_ycbcr_conversion_linear_filter: bool,
    pub sampled_image_ycbcr_conversion_separate_reconstruction_filter: bool,
    pub sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: bool,
    pub sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: bool,
    pub disjoint: bool,
    pub cosited_chroma_samples: bool,
    pub sampled_image_filter_cubic_img: bool,
    pub sampled_image_filter_minmax_ext: bool,
    pub fragment_density_map_ext: bool,
}

#[doc(hidden)]
pub type RawVkFormatFeatureFlags = u32;

impl VkWrappedType<RawVkFormatFeatureFlags> for VkFormatFeatureFlags {
    fn vk_to_raw(src: &VkFormatFeatureFlags, dst: &mut RawVkFormatFeatureFlags) {
        *dst = 0;
        if src.sampled_image { *dst |= 0x00000001; }
        if src.storage_image { *dst |= 0x00000002; }
        if src.storage_image_atomic { *dst |= 0x00000004; }
        if src.uniform_texel_buffer { *dst |= 0x00000008; }
        if src.storage_texel_buffer { *dst |= 0x00000010; }
        if src.storage_texel_buffer_atomic { *dst |= 0x00000020; }
        if src.vertex_buffer { *dst |= 0x00000040; }
        if src.color_attachment { *dst |= 0x00000080; }
        if src.color_attachment_blend { *dst |= 0x00000100; }
        if src.depth_stencil_attachment { *dst |= 0x00000200; }
        if src.blit_src { *dst |= 0x00000400; }
        if src.blit_dst { *dst |= 0x00000800; }
        if src.sampled_image_filter_linear { *dst |= 0x00001000; }
        if src.transfer_src { *dst |= 0x00004000; }
        if src.transfer_dst { *dst |= 0x00008000; }
        if src.midpoint_chroma_samples { *dst |= 0x00020000; }
        if src.sampled_image_ycbcr_conversion_linear_filter { *dst |= 0x00040000; }
        if src.sampled_image_ycbcr_conversion_separate_reconstruction_filter { *dst |= 0x00080000; }
        if src.sampled_image_ycbcr_conversion_chroma_reconstruction_explicit { *dst |= 0x00100000; }
        if src.sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable { *dst |= 0x00200000; }
        if src.disjoint { *dst |= 0x00400000; }
        if src.cosited_chroma_samples { *dst |= 0x00800000; }
        if src.sampled_image_filter_cubic_img { *dst |= 0x00002000; }
        if src.sampled_image_filter_minmax_ext { *dst |= 0x00010000; }
        if src.fragment_density_map_ext { *dst |= 0x01000000; }
    }
}

impl VkRawType<VkFormatFeatureFlags> for RawVkFormatFeatureFlags {
    fn vk_to_wrapped(src: &RawVkFormatFeatureFlags) -> VkFormatFeatureFlags {
        VkFormatFeatureFlags {
            sampled_image: (src & 0x00000001) != 0,
            storage_image: (src & 0x00000002) != 0,
            storage_image_atomic: (src & 0x00000004) != 0,
            uniform_texel_buffer: (src & 0x00000008) != 0,
            storage_texel_buffer: (src & 0x00000010) != 0,
            storage_texel_buffer_atomic: (src & 0x00000020) != 0,
            vertex_buffer: (src & 0x00000040) != 0,
            color_attachment: (src & 0x00000080) != 0,
            color_attachment_blend: (src & 0x00000100) != 0,
            depth_stencil_attachment: (src & 0x00000200) != 0,
            blit_src: (src & 0x00000400) != 0,
            blit_dst: (src & 0x00000800) != 0,
            sampled_image_filter_linear: (src & 0x00001000) != 0,
            transfer_src: (src & 0x00004000) != 0,
            transfer_dst: (src & 0x00008000) != 0,
            midpoint_chroma_samples: (src & 0x00020000) != 0,
            sampled_image_ycbcr_conversion_linear_filter: (src & 0x00040000) != 0,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter: (src & 0x00080000) != 0,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: (src & 0x00100000) != 0,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: (src & 0x00200000) != 0,
            disjoint: (src & 0x00400000) != 0,
            cosited_chroma_samples: (src & 0x00800000) != 0,
            sampled_image_filter_cubic_img: (src & 0x00002000) != 0,
            sampled_image_filter_minmax_ext: (src & 0x00010000) != 0,
            fragment_density_map_ext: (src & 0x01000000) != 0,
        }
    }
}

impl Default for VkFormatFeatureFlags {
    fn default() -> VkFormatFeatureFlags {
        VkFormatFeatureFlags {
            sampled_image: false,
            storage_image: false,
            storage_image_atomic: false,
            uniform_texel_buffer: false,
            storage_texel_buffer: false,
            storage_texel_buffer_atomic: false,
            vertex_buffer: false,
            color_attachment: false,
            color_attachment_blend: false,
            depth_stencil_attachment: false,
            blit_src: false,
            blit_dst: false,
            sampled_image_filter_linear: false,
            transfer_src: false,
            transfer_dst: false,
            midpoint_chroma_samples: false,
            sampled_image_ycbcr_conversion_linear_filter: false,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter: false,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: false,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: false,
            disjoint: false,
            cosited_chroma_samples: false,
            sampled_image_filter_cubic_img: false,
            sampled_image_filter_minmax_ext: false,
            fragment_density_map_ext: false,
        }
    }
}

impl VkFormatFeatureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkFormatFeatureFlags {
            sampled_image: false,
            storage_image: false,
            storage_image_atomic: false,
            uniform_texel_buffer: false,
            storage_texel_buffer: false,
            storage_texel_buffer_atomic: false,
            vertex_buffer: false,
            color_attachment: false,
            color_attachment_blend: false,
            depth_stencil_attachment: false,
            blit_src: false,
            blit_dst: false,
            sampled_image_filter_linear: false,
            transfer_src: false,
            transfer_dst: false,
            midpoint_chroma_samples: false,
            sampled_image_ycbcr_conversion_linear_filter: false,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter: false,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: false,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: false,
            disjoint: false,
            cosited_chroma_samples: false,
            sampled_image_filter_cubic_img: false,
            sampled_image_filter_minmax_ext: false,
            fragment_density_map_ext: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkFormatFeatureFlags {
            sampled_image: true,
            storage_image: true,
            storage_image_atomic: true,
            uniform_texel_buffer: true,
            storage_texel_buffer: true,
            storage_texel_buffer_atomic: true,
            vertex_buffer: true,
            color_attachment: true,
            color_attachment_blend: true,
            depth_stencil_attachment: true,
            blit_src: true,
            blit_dst: true,
            sampled_image_filter_linear: true,
            transfer_src: true,
            transfer_dst: true,
            midpoint_chroma_samples: true,
            sampled_image_ycbcr_conversion_linear_filter: true,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter: true,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: true,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: true,
            disjoint: true,
            cosited_chroma_samples: true,
            sampled_image_filter_cubic_img: true,
            sampled_image_filter_minmax_ext: true,
            fragment_density_map_ext: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.sampled_image { 0x00000001 } else { 0 }
        + if self.storage_image { 0x00000002 } else { 0 }
        + if self.storage_image_atomic { 0x00000004 } else { 0 }
        + if self.uniform_texel_buffer { 0x00000008 } else { 0 }
        + if self.storage_texel_buffer { 0x00000010 } else { 0 }
        + if self.storage_texel_buffer_atomic { 0x00000020 } else { 0 }
        + if self.vertex_buffer { 0x00000040 } else { 0 }
        + if self.color_attachment { 0x00000080 } else { 0 }
        + if self.color_attachment_blend { 0x00000100 } else { 0 }
        + if self.depth_stencil_attachment { 0x00000200 } else { 0 }
        + if self.blit_src { 0x00000400 } else { 0 }
        + if self.blit_dst { 0x00000800 } else { 0 }
        + if self.sampled_image_filter_linear { 0x00001000 } else { 0 }
        + if self.transfer_src { 0x00004000 } else { 0 }
        + if self.transfer_dst { 0x00008000 } else { 0 }
        + if self.midpoint_chroma_samples { 0x00020000 } else { 0 }
        + if self.sampled_image_ycbcr_conversion_linear_filter { 0x00040000 } else { 0 }
        + if self.sampled_image_ycbcr_conversion_separate_reconstruction_filter { 0x00080000 } else { 0 }
        + if self.sampled_image_ycbcr_conversion_chroma_reconstruction_explicit { 0x00100000 } else { 0 }
        + if self.sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable { 0x00200000 } else { 0 }
        + if self.disjoint { 0x00400000 } else { 0 }
        + if self.cosited_chroma_samples { 0x00800000 } else { 0 }
        + if self.sampled_image_filter_cubic_img { 0x00002000 } else { 0 }
        + if self.sampled_image_filter_minmax_ext { 0x00010000 } else { 0 }
        + if self.fragment_density_map_ext { 0x01000000 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkFormatFeatureFlags {
            sampled_image: value & 0x00000001 > 0,
            storage_image: value & 0x00000002 > 0,
            storage_image_atomic: value & 0x00000004 > 0,
            uniform_texel_buffer: value & 0x00000008 > 0,
            storage_texel_buffer: value & 0x00000010 > 0,
            storage_texel_buffer_atomic: value & 0x00000020 > 0,
            vertex_buffer: value & 0x00000040 > 0,
            color_attachment: value & 0x00000080 > 0,
            color_attachment_blend: value & 0x00000100 > 0,
            depth_stencil_attachment: value & 0x00000200 > 0,
            blit_src: value & 0x00000400 > 0,
            blit_dst: value & 0x00000800 > 0,
            sampled_image_filter_linear: value & 0x00001000 > 0,
            transfer_src: value & 0x00004000 > 0,
            transfer_dst: value & 0x00008000 > 0,
            midpoint_chroma_samples: value & 0x00020000 > 0,
            sampled_image_ycbcr_conversion_linear_filter: value & 0x00040000 > 0,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter: value & 0x00080000 > 0,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: value & 0x00100000 > 0,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: value & 0x00200000 > 0,
            disjoint: value & 0x00400000 > 0,
            cosited_chroma_samples: value & 0x00800000 > 0,
            sampled_image_filter_cubic_img: value & 0x00002000 > 0,
            sampled_image_filter_minmax_ext: value & 0x00010000 > 0,
            fragment_density_map_ext: value & 0x01000000 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkFormatFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkFormatFeatureFlags {
            $($x: true,)*
            ..VkFormatFeatureFlags::none()
        }
    }
}