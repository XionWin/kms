#[repr(u32)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum SurfaceFlags
{
    Scanout = (1 << 0),
    Cursor64x64 = (1 << 1),
    Rendering = (1 << 2),
    Write = (1 << 3),
    Linear = (1 << 4),
}