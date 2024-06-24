use super::GbmBufferObject;

#[repr(C)]
pub struct GbmSurface
{
}


#[link(name = "gbm")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn gbm_surface_create(device: *const crate::ffi::GbmDevice, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> *const GbmSurface;
    pub fn gbm_surface_create_with_modifiers(device: *const crate::ffi::GbmDevice, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, modifiers: *const crate::def::FormatModifier, count: libc::c_uint) -> *const GbmSurface;
    
    pub fn gbm_surface_release_buffer(handle: *const GbmSurface, bo: *const GbmBufferObject);
    pub fn gbm_surface_lock_front_buffer(handle: *const GbmSurface) -> *const GbmBufferObject;
    pub fn gbm_surface_destroy(handle: *const GbmSurface);
}