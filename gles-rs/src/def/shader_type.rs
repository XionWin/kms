#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    FragmentShader = gl::FRAGMENT_SHADER,
    VertexShader = gl::VERTEX_SHADER,
}