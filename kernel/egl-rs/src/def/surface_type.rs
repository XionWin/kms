use super::Definition;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum SurfaceType
{
    OpenGL{ version: libc::c_int },
    // OpenGLES = Definition.OPENGL_ES_BIT,
    // OpenOpenGLES = Definition.OPENGL_ES2_BIT,
    // OpenGLESV3 = Definition.OPENGL_ES3_BIT,

    OpenGlesV2,
    OpenGlesV3,
    // Window = Definition::WINDOW_BIT,
}

impl SurfaceType {
    pub fn get_definition(&self) -> libc::c_int {
        match self {
            SurfaceType::OpenGL{version: _} => Definition::OPENGL_BIT,
            SurfaceType::OpenGlesV2 => Definition::OPENGL_ES2_BIT,
            SurfaceType::OpenGlesV3 => Definition::OPENGL_ES3_BIT
        }
    }
    pub fn get_version(&self) -> libc::c_int {
        match self {
            SurfaceType::OpenGL{version} => version.to_owned(),
            SurfaceType::OpenGlesV2 => 2,
            SurfaceType::OpenGlesV3 => 3
        }
    }
}
