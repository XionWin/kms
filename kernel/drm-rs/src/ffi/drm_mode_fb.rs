#[repr(C)]
#[derive(Debug)]
pub struct DrmModeFramebuffer {
    pub fb_id: libc::c_uint,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub pitch: libc::c_uint,
    pub bpp: libc::c_uint,
    pub depth: libc::c_uint,
    /* driver specific handle */
    pub handle: libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetFB(fd: libc::c_int, fb_id: libc::c_uint) -> *const DrmModeFramebuffer;
    pub fn drmModeFreeFB(ptr: *const DrmModeFramebuffer);
    // #[allow(dead_code)]
    // pub fn drmModeAddFB2(
    //     fd: libc::c_int,
    //     width: libc::c_uint,
    //     height: libc::c_uint,
    //     pixel_format: libc::c_uint,
    //     bo_handles: *const *const libc::c_void,
    //     pitches: *const libc::c_uint,
    //     offsets: *const libc::c_uint,
    //     buf_id: *mut libc::c_uint,
    //     flags: libc::c_uint,
    // ) -> libc::c_int;

    
}
