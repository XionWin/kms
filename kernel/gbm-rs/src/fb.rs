pub fn get_fb2(
    fd: libc::c_int,
    width: libc::c_uint,
    height: libc::c_uint,
    pixel_format: libc::c_uint,
    bo_handles: *const *const libc::c_void,
    strides: *const libc::c_uint,
    offsets: *const libc::c_uint,
    flags: libc::c_uint,
) -> libc::c_uint {
    unsafe {
        let mut buf_id = 0;
        match crate::ffi::drmModeAddFB2(
            fd,
            width,
            height,
            pixel_format,
            bo_handles,
            strides,
            offsets,
            &mut buf_id,
            flags,
        ) {
            r if r == 0 => buf_id,
            _ => panic!("[DRM] drmModeAddFB2 failed"),
        }
    }
}
