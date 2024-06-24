const DRM_NAME_STRING_LEN: usize = 32usize;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrmModeProperty {
    pub(crate) prop_id: libc::c_uint,
    pub(crate) flags: libc::c_uint,
    pub(crate) name: [libc::c_char; DRM_NAME_STRING_LEN],
    pub(crate) count_values: libc::c_int,
	pub(crate) values: *const libc::c_ulong,
	pub(crate) count_enums: libc::c_int,
    pub(crate) enums: *const DrmModePropertyEnum
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrmModePropertyEnum {
	pub(crate) value: libc::c_ulong,
    pub(crate) name: [libc::c_char; DRM_NAME_STRING_LEN],
}


#[link(name = "drm")]
#[allow(improper_ctypes, dead_code)]
extern "C" {
    pub(crate) fn drmModeGetProperty(fd: libc::c_int, property_id: libc::c_uint) -> *const DrmModeProperty;
    pub(crate) fn drmModeConnectorSetProperty(fd: libc::c_int, connector_id: libc::c_uint, property_id: libc::c_uint, value: libc::c_uint) -> libc::c_int;
    pub(crate) fn drmModeFreeProperty (ptr: *const DrmModeProperty);
}
