#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = gl::ARRAY_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
}