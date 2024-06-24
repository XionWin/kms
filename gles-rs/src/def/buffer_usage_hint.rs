#[repr(u32)]
pub enum BufferUsageHint {
    StreamDraw = crate::ffi::GL_STREAM_DRAW,
    StaticDraw = crate::ffi::GL_STATIC_DRAW,
    DynamicDraw = crate::ffi::GL_DYNAMIC_DRAW,
}