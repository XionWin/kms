use libc::{c_uint, c_uchar, c_void, c_char, c_short, c_int, c_ushort, intptr_t, ssize_t};


// Types

pub type GLvoid = c_void /* unknown kind Void */;

pub type GLchar = c_char;

pub type GLenum = c_uint;

pub type GLboolean = c_uchar;

pub type GLbitfield = c_uint;

pub type GLbyte = i8;

pub type GLshort = c_short;

pub type GLint = c_int;

pub type GLsizei = c_int;

pub type GLubyte = u8;

pub type GLushort = c_ushort;

pub type GLuint = c_uint;

pub type GLfloat = f32;

pub type GLclampf = f32;

pub type GLfixed = i32;

pub type GLintptr = intptr_t;

pub type GLsizeiptr = ssize_t;

// gl2ext
pub type GLeglImageOES = *mut c_void;
