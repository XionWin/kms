#[repr(C)]
pub struct EglDisplayRaw {} 
#[repr(C)]
pub struct EglConfigRaw {} 
#[repr(C)]
pub struct EglContextRaw {} 
#[repr(C)]
pub struct EGLNativeWindowTypeRaw {} 
#[repr(C)]
pub struct EglSurfaceRaw {} 

pub type EglDisplay = *const EglDisplayRaw;
pub type EglConfig = *const EglConfigRaw;
pub type EglContext = *const EglContextRaw;
pub type EGLNativeWindowType = *const EGLNativeWindowTypeRaw;
pub type EglSurface = *const EglSurfaceRaw;

#[link(name = "EGL")]
#[allow(improper_ctypes, dead_code)]
extern "C" {
    pub fn eglGetError() -> crate::def::ErrorCode;
    pub fn eglQueryString(display: EglDisplay, name: libc::c_int) -> *const libc::c_char;
    pub fn eglGetProcAddress(func_name: *const libc::c_char) -> *const fn(libc::c_uint, libc::c_uint, *const libc::c_uint) -> EglDisplay;
    pub fn eglInitialize(display: EglDisplay, major: *mut libc::c_int, minor: *mut libc::c_int) -> bool;
    pub fn eglBindAPI(api: crate::def::RenderAPI) -> bool;

    pub fn eglChooseConfig(display: EglDisplay, attrib_list: *const libc::c_int, configs: EglConfig, config_size: libc::c_int, num_config: *mut libc::c_int) -> bool;
    pub fn eglCreateContext(display: EglDisplay, config: EglConfig, share_context: EglContext, attrib_list: *const libc::c_int) -> EglContext;

    pub fn eglCreateWindowSurface(display: EglDisplay, config: EglConfig, native_window: EGLNativeWindowType, attrib_list: *const libc::c_int) -> EglSurface;
    
    pub fn eglSwapBuffers(display: EglDisplay, surface: EglSurface) -> bool;
    
    pub fn eglMakeCurrent(display: EglDisplay, draw: EglSurface, read: EglSurface, context: EglContext) -> bool;
}