#[repr(u32)]
pub enum StringName {
    Vendor = gl::VENDOR,
    Renderer = gl::RENDERER,
    Version = gl::VERSION,
    Extensions = gl::EXTENSIONS,
    ShadingLanguageVersion = gl::SHADING_LANGUAGE_VERSION,
}