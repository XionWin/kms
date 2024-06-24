#[repr(u32)]
pub enum VertexAttribPointerType {
	Byte = crate::ffi::GL_BYTE,
	UnsignedByte = crate::ffi::GL_UNSIGNED_BYTE,
	Short = crate::ffi::GL_SHORT,
	UnsignedShort = crate::ffi::GL_UNSIGNED_SHORT,
	Int = crate::ffi::GL_INT,
	UnsignedInt = crate::ffi::GL_UNSIGNED_INT,
	Float = crate::ffi::GL_FLOAT,
	Fixed = crate::ffi::GL_FIXED
}