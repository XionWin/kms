#[repr(u32)]
pub enum BufferUsageHint {
    StreamDraw = gl::STREAM_DRAW,
    StaticDraw = gl::STATIC_DRAW,
    DynamicDraw = gl::DYNAMIC_DRAW,
}