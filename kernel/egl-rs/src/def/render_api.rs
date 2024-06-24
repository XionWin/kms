#[repr(i32)]
pub enum RenderAPI {
    GLES = super::Definition::OPENGL_ES_API,
    GL = super::Definition::OPENGL_API,
    VG = super::Definition::OPENVG_API
}