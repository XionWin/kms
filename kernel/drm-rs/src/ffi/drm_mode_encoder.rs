#[repr(C)]
#[derive(Debug)]
pub struct DrmModeEncoder
{
    pub encoder_id: libc::c_uint,
    pub encoder_type: crate::ffi::enums::EncoderType,
    pub crtc_id: libc::c_uint,
    pub possible_crtcs: libc::c_uint,
    pub possible_clones: libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetEncoder(fd: libc::c_int, encoder_id: libc::c_uint) -> *const DrmModeEncoder;
    pub fn drmModeFreeEncoder(handle: *const DrmModeEncoder);
}