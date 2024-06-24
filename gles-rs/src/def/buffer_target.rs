#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = crate::ffi::GL_ARRAY_BUFFER,
    ElementArrayBuffer = crate::ffi::GL_ELEMENT_ARRAY_BUFFER,
}