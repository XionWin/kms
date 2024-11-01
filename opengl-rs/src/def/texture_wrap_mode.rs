#[repr(u32)]
pub enum TextureWrapMode {
    Clamp = 0x2900,
    Repeat = 0x2901,
    ClampToBorder = 0x812D,
    ClampToEdge = 0x812F,
    MirroredRepeat = 0x8370,
}