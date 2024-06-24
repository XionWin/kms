use libc::*;

use crate::def::{TextureUnit, TextureMinFilter};

#[derive(Debug)]
pub struct GfxTexture
{
    pub id: c_uint,
    pub texture_unit: TextureUnit,
    pub texture_min_filter: TextureMinFilter,
}

#[derive(Debug)]
pub struct ImageData
{
    pub width: c_int,
    pub height: c_int,
    pub value: Vec<u8>,
}

impl GfxTexture {
    pub fn new(texture_unit: TextureUnit, texture_min_filter: TextureMinFilter) -> Self {
        Self {
            id: crate::gen_texture(),
            texture_unit,
            texture_min_filter
        }
    }

    pub fn load(&self, image_data: &ImageData) {
        crate::active_texture(self.texture_unit);
        crate::bind_texture(crate::def::TextureTarget::Texture2D, self.id);

        crate::tex_image_2d(crate::def::TextureTarget::Texture2D, 0, crate::def::PixelInternalFormat::Rgba, image_data.width, image_data.height, 0, crate::def::PixelFormat::Rgba, crate::def::PixelType::UnsignedByte, Some(&image_data.value));

        // Now that our texture is loaded, we can set a few settings to affect how the image appears on rendering.

        // First, we set the min and mag filter. These are used for when the texture is scaled down and up, respectively.
        // Here, we use Linear for both. This means that OpenGL will try to blend pixels, meaning that textures scaled too far will look blurred.
        // You could also use (amongst other options) Nearest, which just grabs the nearest pixel, which makes the texture look pixelated if scaled too far.
        // NOTE: The default settings for both of these are LinearMipmap. If you leave these as default but don't generate mipmaps,
        // your image will fail to render at all (usually resulting in pure black instead).
        crate::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureMinFilter, self.texture_min_filter as _);
        crate::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureMagFilter, self.texture_min_filter as _);

        // Now, set the wrapping mode. S is for the X axis, and T is for the Y axis.
        // We set this to Repeat so that textures will repeat when wrapped. Not demonstrated here since the texture coordinates exactly match
        crate::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureWrapS, crate::def::TextureWrapMode::Repeat as _);
        crate::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureWrapT, crate::def::TextureWrapMode::Repeat as _);

        // Next, generate mipmaps.
        // Mipmaps are smaller copies of the texture, scaled down. Each mipmap level is half the size of the previous one
        // Generated mipmaps go all the way down to just one pixel.
        // OpenGL will automatically switch between mipmaps when an object gets sufficiently far away.
        // This prevents moiré effects, as well as saving on texture bandwidth.
        // Here you can see and read about the morié effect https://en.wikipedia.org/wiki/Moir%C3%A9_pattern
        // Here is an example of mips in action https://en.wikipedia.org/wiki/File:Mipmap_Aliasing_Comparison.png
        crate::generate_mipmap(crate::def::GenerateMipmapTarget::Texture2D);
    }
}

impl Drop for GfxTexture {
    fn drop(&mut self) {
        crate::delete_texture(self.id);
    }
}