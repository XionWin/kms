impl crate::GfxShader {
    pub(crate) fn load(self) -> Self {
        let mut source = (&self.source).bytes().collect::<Vec<_>>();
        source.push(b'\0');
        let sources = vec![source.as_ptr()];
        unsafe {
            crate::ffi::glShaderSource(self.id, 1, sources.as_ptr(), std::ptr::null());
            crate::ffi::glCompileShader(self.id);
        }
        check_compile(self)
    }
}

fn check_compile(shader: super::GfxShader) -> super::GfxShader {
    let mut is_compiled = 0;
    unsafe {
        crate::ffi::glGetShaderiv(shader.id, crate::ffi::GL_COMPILE_STATUS, &mut is_compiled);
    }
    if is_compiled == 0 {
        panic!("GLES shader compile faild");
    }
    return shader;
}
