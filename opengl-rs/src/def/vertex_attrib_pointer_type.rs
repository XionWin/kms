#[repr(u32)]
pub enum VertexAttribPointerType {
	Byte = gl::BYTE,
	UnsignedByte = gl::UNSIGNED_BYTE,
	Short = gl::SHORT,
	UnsignedShort = gl::UNSIGNED_SHORT,
	Int = gl::INT,
	UnsignedInt = gl::UNSIGNED_INT,
	Float = gl::FLOAT,
	Fixed = gl::FIXED
}