use std::os::unix::prelude::RawFd;
use crate::models::EventContext;


pub fn vertical_synchronize(fd: RawFd, crtc_id: libc::c_uint, fb: libc::c_uint) {
    let evt_context = EventContext {
        version: DRM_CONTEXT_VERSION,
        vblank_handler,
        page_flip_handler,
    };

    let mut user_data = 1;
    match page_flip( fd, crtc_id, fb as _, crate::enums::PageFlipFlags::FLIP_EVENT, &mut user_data as *mut libc::c_int as _) {
        result if result != 0 => panic!("page_flip error"),
        _ => {}
    }

    while user_data != 0 {
        let r = handle_event(fd, &evt_context);
        if r != 0 {
            panic!("handle_event_result: {:?}", r);
        }
    }
}

pub fn page_flip(
    fd: RawFd,
    crtc_id: libc::c_uint,
    fb_id: libc::c_uint,
    flags: crate::enums::PageFlipFlags,
    user_data: *mut libc::c_void,
) -> libc::c_int {
    unsafe { crate::ffi::drmModePageFlip(fd, crtc_id, fb_id, flags, user_data) }
}

pub fn handle_event(fd: RawFd, evt_context: *const crate::models::EventContext) -> libc::c_int {
    unsafe { crate::ffi::drmHandleEvent(fd, evt_context) }
}

const DRM_CONTEXT_VERSION: libc::c_int = 2;
extern fn vblank_handler(
    _fd: libc::c_int,
    _sequence: libc::c_uint,
    _tv_sec: libc::c_uint,
    _tv_usec: libc::c_uint,
    _user_data: *mut libc::c_void,
) {}
extern fn page_flip_handler(
    _fd: libc::c_int,
    _sequence: libc::c_uint,
    _tv_sec: libc::c_uint,
    _tv_usec: libc::c_uint,
    user_data: *mut libc::c_void,
) {
    unsafe {
        *(user_data as *mut libc::c_int) = 0;
    }
}