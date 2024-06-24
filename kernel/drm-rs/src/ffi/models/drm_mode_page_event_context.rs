#[repr(C)]
pub struct EventContext {
    pub version: libc::c_int,
    pub vblank_handler: extern fn(fd: libc::c_int,
                                  sequence: libc::c_uint,
                                  tv_sec: libc::c_uint,
                                  tv_usec: libc::c_uint,
                                  user_data: *mut libc::c_void),

    pub page_flip_handler: extern fn(fd: libc::c_int,
                                     sequence: libc::c_uint,
                                     tv_sec: libc::c_uint,
                                     tv_usec: libc::c_uint,
                                     user_data: *mut libc::c_void),
}

