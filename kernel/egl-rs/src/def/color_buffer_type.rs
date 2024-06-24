bitflags! {
    #[repr(C)]
    pub struct ColorBufferType: u32 {
        const RGB_BUFFER = 0x308E;
        const LUMINANCE_BUFFER = 0x308F;
    }
}