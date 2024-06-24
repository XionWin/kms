#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    FragmentShader = crate::ffi::GL_FRAGMENT_SHADER,
    VertexShader = crate::ffi::GL_VERTEX_SHADER,
}