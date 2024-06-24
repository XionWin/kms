#[link(name = "drm")]
extern "C" {
    pub fn drmModeAddFB2(
        fd: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
        pixel_format: libc::c_uint,
        bo_handles: *const *const libc::c_void,
        pitches: *const libc::c_uint,
        offsets: *const libc::c_uint,
        buf_id: *mut libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
}
