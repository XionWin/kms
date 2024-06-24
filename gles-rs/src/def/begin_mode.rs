#[repr(u32)]
pub enum BeginMode {
    Points = crate::ffi::GL_POINTS,
    Lines = crate::ffi::GL_LINES,
    LineLoop = crate::ffi::GL_LINE_LOOP,
    LineStrip = crate::ffi::GL_LINE_STRIP,
    Triangles = crate::ffi::GL_TRIANGLES,
    TriangleStrip = crate::ffi::GL_TRIANGLE_STRIP,
    TriangleFan = crate::ffi::GL_TRIANGLE_FAN,
}