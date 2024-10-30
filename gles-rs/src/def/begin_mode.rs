#[repr(u32)]
pub enum BeginMode {
    Points = gl::POINTS,
    Lines = gl::LINES,
    LineLoop = gl::LINE_LOOP,
    LineStrip = gl::LINE_STRIP,
    Triangles = gl::TRIANGLES,
    TriangleStrip = gl::TRIANGLE_STRIP,
    TriangleFan = gl::TRIANGLE_FAN,
}