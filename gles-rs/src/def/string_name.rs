#[repr(u32)]
pub enum StringName {
    Vendor = crate::ffi::GL_VENDOR,
    Renderer = crate::ffi::GL_RENDERER,
    Version = crate::ffi::GL_VERSION,
    Extensions = crate::ffi::GL_EXTENSIONS,
    ShadingLanguageVersion = crate::ffi::GL_SHADING_LANGUAGE_VERSION,
}