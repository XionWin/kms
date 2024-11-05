use crate::{def::StringName, LibraryLoader};
use std::{
    ffi::{CStr, CString, c_char, c_float, c_int, c_uint},
    str::from_utf8,
};

pub fn load() {
    let loader = LibraryLoader::new("libGLESv2.so.2");
    gl::load_with(|name| loader.get_proc_address(name));
}

pub fn get_string(name: StringName) -> Option<String> {
    unsafe {
        let c_str = gl::GetString(name as _);

        if !c_str.is_null() {
            match from_utf8(CStr::from_ptr(c_str).to_bytes()) {
                Ok(s) => Some(s.to_string()),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub fn enable(enale_cap: crate::def::EnableCap) {
    unsafe {
        gl::Enable(enale_cap as _);
    }
}

pub fn disable(enale_cap: crate::def::EnableCap) {
    unsafe {
        gl::Disable(enale_cap as _);
    }
}

pub fn create_program() -> c_uint {
    unsafe { gl::CreateProgram() }
}

pub fn delete_program(program_id: c_uint) {
    unsafe { gl::DeleteProgram(program_id) }
}

pub fn create_shader(shader_type: crate::def::ShaderType) -> c_uint {
    unsafe { gl::CreateShader(shader_type as _) }
}

pub fn delete_shader(shader_id: c_uint) {
    unsafe { gl::DeleteShader(shader_id) }
}

pub fn shader_source(shader_id: c_uint, source_code: &str) {
    let cstr = std::ffi::CString::new(source_code).unwrap();
    let sources = vec![cstr.as_ptr()];
    unsafe { gl::ShaderSource(shader_id, 1, sources.as_ptr(), std::ptr::null()) }
}

pub fn compile_shader(shader_id: c_uint) {
    unsafe { gl::CompileShader(shader_id) }
}

pub fn get_shaderiv(shader_id: c_uint) -> c_int {
    let mut is_compiled = 0;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut is_compiled);
    }
    is_compiled
}

pub fn attach_shader(program_id: c_uint, shader_id: c_uint) {
    unsafe { gl::AttachShader(program_id, shader_id) }
}

pub fn link_program(program_id: c_uint) {
    unsafe { gl::LinkProgram(program_id) }
}

pub fn use_program(program_id: c_uint) {
    unsafe { gl::UseProgram(program_id) }
}

pub fn bind_attrib_location(program_id: c_uint, index: c_uint, name: &str) {
    let c_str = CString::new(name).unwrap();
    unsafe { gl::BindAttribLocation(program_id, index, c_str.as_ptr()) }
}

pub fn check_link_status(program_id: c_uint) {
    let mut is_linked = 0;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut is_linked);
    }
    if is_linked == 0 {
        match get_program_information(program_id) {
            Some(msg) => panic!("GLES program link faild error: {:?}", msg),
            None => panic!("GLES program link faild error: NONE"),
        }
    }
}

pub fn get_program_information(program_id: c_uint) -> Option<String> {
    let mut len = 0;
    unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
    }
    match len {
        len if len > 0 => {
            let mut buf = vec![0u8; len as _];
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut::<libc::c_int>(),
                    buf.as_mut_ptr(),
                );
            }
            Some(String::from_utf8(buf).expect("GLES glGetProgramInfoLog error"))
        }
        _ => None,
    }
}

pub fn get_shader_information(shader_id: c_uint) -> Option<String> {
    let mut len = 0;
    unsafe {
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
    }
    match len {
        len if len > 0 => {
            let mut buf = vec![0u8; len as _];
            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut::<libc::c_int>(),
                    buf.as_mut_ptr(),
                );
            }
            Some(String::from_utf8(buf).expect("GLES glGetShaderInfoLog error"))
        }
        _ => None,
    }
}

pub fn get_uniform_location(program_id: c_uint, name: &str) -> c_int {
    unsafe {
        let c_str = CString::new(name).unwrap();
        gl::GetUniformLocation(program_id, c_str.as_ptr() as *const c_char)
    }
}

pub fn uniform2i(location: c_int, x: c_int, y: c_int) {
    unsafe { gl::Uniform2i(location, x, y) }
}

pub fn uniform2f(location: c_int, x: c_float, y: c_float) {
    unsafe { gl::Uniform2f(location, x, y) }
}

pub fn uniform_1i(location: c_int, v: c_int) {
    unsafe { gl::Uniform1i(location, v) }
}

pub fn uniform_1f(location: c_int, v: c_float) {
    unsafe { gl::Uniform1f(location, v) }
}

pub fn viewport(x: c_int, y: c_int, width: c_int, height: c_int) {
    unsafe { gl::Viewport(x, y, width, height) }
}

pub fn clear(mask: c_uint) {
    unsafe { gl::Clear(mask) }
}

pub fn clear_color(red: c_float, green: c_float, blue: c_float, alpha: c_float) {
    unsafe { gl::ClearColor(red, green, blue, alpha) }
}

pub fn gen_vertex_arrays(n: c_int, array: *mut c_uint) {
    unsafe {
        gl::GenVertexArrays(n, array);
    }
}

pub fn gen_vertex_array() -> c_uint {
    let mut buffer = 0u32;
    unsafe {
        gl::GenVertexArrays(1, &mut buffer);
    }
    buffer
}

pub fn bind_vertex_array(array_id: c_uint) {
    unsafe {
        gl::BindVertexArray(array_id);
    }
}

pub fn gen_buffers(n: c_int) -> Vec<c_uint> {
    let mut buffer = std::vec::from_elem(0, n as _);
    unsafe {
        gl::GenBuffers(n, buffer.as_mut_ptr());
    }
    buffer
}

pub fn gen_buffer() -> c_uint {
    let mut buffer = 0u32;
    unsafe {
        gl::GenBuffers(1, &mut buffer);
    }
    buffer
}

pub fn bind_buffer(target: crate::def::BufferTarget, buffer_id: c_uint) {
    unsafe {
        gl::BindBuffer(target as _, buffer_id);
    }
}

pub fn buffer_data<T>(
    target: crate::def::BufferTarget,
    data: &[T],
    usage: crate::def::BufferUsageHint,
) {
    unsafe {
        gl::BufferData(
            target as _,
            (data.len() * std::mem::size_of::<T>()) as _,
            data.as_ptr() as *const _,
            usage as _,
        );
    }
}

pub fn delete_buffers(n: c_int, buffer_ids: &[c_uint]) {
    unsafe {
        gl::DeleteBuffers(n, buffer_ids.as_ptr());
    }
}

pub fn delete_buffer(buffer_id: c_uint) {
    delete_buffers(1, &vec![buffer_id])
}

pub fn get_attrib_location(program_id: c_uint, name: &str) -> c_uint {
    let mut buffer = name.bytes().collect::<Vec<u8>>();
    buffer.push(b'\0');
    match unsafe { gl::GetAttribLocation(program_id, buffer.as_ptr()) } {
        value if value >= 0 => value as c_uint,
        _ => panic!("GLES get_attrib_location error"),
    }
}

pub fn vertex_attrib_pointer_f32(
    index: c_uint,
    size: c_int,
    normalized: bool,
    stride: c_int,
    offset: c_uint,
) {
    unsafe { gl::VertexAttribPointer(index, size, gl::FLOAT, normalized as _, stride, offset as _) }
}

pub fn enable_vertex_attrib_array(index: c_uint) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

pub fn draw_arrays(begin_mode: crate::def::BeginMode, first: c_int, count: c_int) {
    unsafe {
        gl::DrawArrays(begin_mode as _, first, count);
    }
}

pub fn draw_elements<T>(
    begin_mode: crate::def::BeginMode,
    count: c_int,
    draw_elements_type: crate::def::DrawElementsType,
    indices: Option<&[T]>,
) {
    unsafe {
        gl::DrawElements(
            begin_mode as _,
            match indices {
                Some(i) => std::cmp::min(count, i.len() as _),
                None => count,
            },
            draw_elements_type as _,
            match indices {
                Some(i) => i.as_ptr() as *const _,
                None => std::ptr::null(),
            },
        )
    }
}

pub fn gen_textures(n: c_int) -> Vec<c_uint> {
    unsafe {
        let mut result = std::vec::from_elem(0, n as _);
        gl::GenTextures(n, result.as_mut_ptr());
        return result;
    }
}

pub fn gen_texture() -> c_uint {
    gen_textures(1)[0]
}

pub fn delete_textures(textures: &mut [c_uint]) {
    unsafe { gl::DeleteTextures(textures.len() as _, textures.as_mut_ptr()) }
}

pub fn delete_texture(texture: c_uint) {
    delete_textures(&mut vec![texture])
}

pub fn active_texture(texture_unit: crate::def::TextureUnit) {
    unsafe {
        gl::ActiveTexture(texture_unit as _);
    }
}

pub fn bind_texture(texture_target: crate::def::TextureTarget, texture_id: c_uint) {
    unsafe {
        gl::BindTexture(texture_target as _, texture_id);
    }
}

pub fn pixel_storei(pname: crate::def::ALL, param: c_int) {
    unsafe {
        gl::PixelStorei(pname as _, param);
    }
}

// FIXME: Does not verify buffer size -- unsafe!
pub fn tex_image_2d<T>(
    texture_target: crate::def::TextureTarget,
    level: c_int,
    internal_format: crate::def::PixelInternalFormat,
    width: c_int,
    height: c_int,
    border: c_int,
    format: crate::def::PixelFormat,
    ty: crate::def::PixelType,
    opt_data: Option<&[T]>,
) {
    match opt_data {
        Some(data) => unsafe {
            let pdata = data.as_ptr() as *const _;
            gl::TexImage2D(
                texture_target as _,
                level,
                internal_format as _,
                width,
                height,
                border,
                format as _,
                ty as _,
                pdata,
            );
        },
        None => unsafe {
            gl::TexImage2D(
                texture_target as _,
                level,
                internal_format as _,
                width,
                height,
                border,
                format as _,
                ty as _,
                std::ptr::null(),
            );
        },
    }
}

pub fn tex_sub_image_2d<T>(
    texture_target: crate::def::TextureTarget,
    level: c_int,
    xoffset: c_int,
    yoffset: c_int,
    width: c_int,
    height: c_int,
    format: crate::def::PixelFormat,
    ty: crate::def::PixelType,
    opt_data: Option<&[T]>,
) {
    match opt_data {
        Some(data) => unsafe {
            let pdata = data.as_ptr() as *const _;
            gl::TexSubImage2D(
                texture_target as _,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format as _,
                ty as _,
                pdata,
            );
        },
        None => unsafe {
            gl::TexSubImage2D(
                texture_target as _,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format as _,
                ty as _,
                std::ptr::null(),
            );
        },
    }
}

pub fn tex_parameter_i(
    texture_target: crate::def::TextureTarget,
    texture_param_name: crate::def::TextureParameterName,
    value: c_int,
) {
    unsafe { gl::TexParameteri(texture_target as _, texture_param_name as _, value) }
}

pub fn generate_mipmap(generate_mipmap_target: crate::def::GenerateMipmapTarget) {
    unsafe { gl::GenerateMipmap(generate_mipmap_target as _) }
}

pub fn blend_func(sfactor: crate::def::BlendingFactor, dfactor: crate::def::BlendingFactor) {
    unsafe { gl::BlendFunc(sfactor as _, dfactor as _) }
}

pub fn stencil_mask(mask: c_uint) {
    unsafe { gl::StencilMask(mask) }
}

pub fn stencil_func(func: crate::def::ALL, reference: c_int, mask: c_uint) {
    unsafe { gl::StencilFunc(func as _, reference, mask) }
}

pub fn color_mask(red: bool, green: bool, blue: bool, alpha: bool) {
    unsafe { gl::ColorMask(red as _, green as _, blue as _, alpha as _) }
}

pub fn stencil_op(fail: crate::def::ALL, zfail: crate::def::ALL, zpass: crate::def::ALL) {
    unsafe { gl::StencilOp(fail as _, zfail as _, zpass as _) }
}

pub fn stencil_op_separate(
    face: crate::def::ALL,
    fail: crate::def::ALL,
    zfail: crate::def::ALL,
    zpass: crate::def::ALL,
) {
    unsafe { gl::StencilOpSeparate(face as _, fail as _, zfail as _, zpass as _) }
}

pub fn flush() {
    unsafe { gl::Flush() }
}

pub fn finish() {
    unsafe { gl::Finish() }
}
