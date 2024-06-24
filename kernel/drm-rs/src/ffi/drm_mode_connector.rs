#[repr(C)]
#[derive(Debug)]
pub struct DrmModeConnector {
    pub connector_id: libc::c_uint,
    pub encoder_id: libc::c_uint,
    pub connector_type: crate::ffi::enums::ConnectorType,
    pub connector_type_id: libc::c_uint,
    pub connection: crate::ffi::enums::ConnectionStatus,
    
    pub mm_width: libc::c_uint,
    pub mm_height: libc::c_uint,
    pub subpixel: crate::ffi::enums::SubPixel, 

    pub count_modes: libc::c_int,
    pub modes: *const crate::ffi::models::DrmModeInfo,

    pub count_props: libc::c_int,
    pub props: *const libc::c_uint,
    pub prop_values: *const libc::c_ulong,

    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetConnector(fd: libc::c_int, connector_id: libc::c_uint) -> *const DrmModeConnector;
    pub fn drmModeFreeConnector(handle: *const DrmModeConnector);
}
