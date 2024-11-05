use super::{TextureMagFilter, TextureMinFilter};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum TextureFilter
{
    Nearest = 9728,
    Linear = 9729,
}

impl TextureFilter {
    pub fn to_texture_min_filter(&self) -> TextureMinFilter {
        match self {
            TextureFilter::Nearest => TextureMinFilter::NearestMipmapNearest,
            TextureFilter::Linear => TextureMinFilter::LinearMipmapLinear,
        }
    }
    pub fn to_texture_mag_filter(&self) -> TextureMagFilter {
        match self {
            TextureFilter::Nearest => TextureMagFilter::Nearest,
            TextureFilter::Linear => TextureMagFilter::Linear,
        }
    }
}
