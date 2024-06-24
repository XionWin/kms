use libc::*;

#[derive(Debug)]
pub struct GfxShader
{
    pub id: c_uint,
    pub source: String,
    pub shader_type: crate::def::ShaderType,
}

impl GfxShader {
    pub fn new(shader_type: crate::def::ShaderType, path: &str) -> Self {
        Self {
            id: unsafe {
                crate::ffi::glCreateShader(shader_type as _)
            },
            source: std::fs::read_to_string(path)
            .expect("unread the file"),
            shader_type,
        }
    }
}

impl Drop for GfxShader {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::glDeleteShader(self.id);
        }
    }
}