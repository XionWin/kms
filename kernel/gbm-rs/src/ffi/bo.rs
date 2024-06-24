#[repr(C)]
pub union GbmBufferObjectHandle {
    pub ptr: *const libc::c_void,
    pub s32: libc::c_int,
    pub u32_: libc::c_uint,
    pub s64: libc::c_longlong,
    pub u64_: libc::c_ulonglong,
}

#[repr(C)]
pub struct GbmBufferObject
{
}

#[link(name = "gbm")]
#[allow(improper_ctypes)]
extern "C" {
    // pub fn gbm_bo_create(handle: *const crate::ffi::GbmDevice, width: libc::c_uint, height: libc::c_uint, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> *const GbmBufferObject;
    pub fn gbm_bo_get_handle_for_plane(handle: *const GbmBufferObject, plane: libc::c_int) -> GbmBufferObjectHandle;
    // pub fn gbm_bo_get_handle(handle: *const GbmBufferObject) -> GbmBufferObjectHandle;

    pub fn gbm_bo_get_user_data(handle: *const GbmBufferObject) -> *const libc::c_void;
    pub fn gbm_bo_get_plane_count(handle: *const GbmBufferObject) -> libc::c_int;

    
    pub fn gbm_bo_get_width(handle: *const GbmBufferObject) -> libc::c_uint;
    pub fn gbm_bo_get_height(handle: *const GbmBufferObject) -> libc::c_uint;
    // pub fn gbm_bo_get_stride(handle: *const GbmBufferObject) -> libc::c_uint;
    pub fn gbm_bo_get_format(handle: *const GbmBufferObject) -> crate::def::SurfaceFormat;
    // pub fn gbm_bo_get_modifier(handle: *const GbmBufferObject) -> libc::c_ulonglong;
    pub fn gbm_bo_get_stride_for_plane(handle: *const GbmBufferObject, plane: libc::c_int) -> libc::c_uint;
    pub fn gbm_bo_get_offset(handle: *const GbmBufferObject, plane: libc::c_int) -> libc::c_uint;

    pub fn gbm_bo_set_user_data(handle: *const GbmBufferObject, data: *const libc::c_void, callback: extern "C" fn(*const GbmBufferObject, *const libc::c_void));

    pub fn gbm_bo_destroy(handle: *const GbmBufferObject);
}