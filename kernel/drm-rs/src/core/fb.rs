#[derive(Debug)]
#[allow(dead_code)]
pub struct Framebuffer {
    handle: *const crate::ffi::DrmModeFramebuffer,
    fb_id: libc::c_uint,
    width: libc::c_uint,
    height: libc::c_uint,
    pitch: libc::c_uint,
    bpp: libc::c_uint,
    depth: libc::c_uint,
    /* driver specific handle */
}

impl Framebuffer {
    pub fn new(fb: &crate::ffi::DrmModeFramebuffer) -> Self {
        Self {
            handle: fb,
            fb_id: fb.fb_id,
            width: fb.width,
            height: fb.height,
            pitch: fb.pitch,
            bpp: fb.bpp,
            depth: fb.depth,
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeFB(self.handle);
            colored_rs::print_debug!("Framebuffer: {:?} droped", self.handle);
        }
    }
}

// pub fn get_fb2(
//     fd: libc::c_int,
//     width: libc::c_uint,
//     height: libc::c_uint,
//     pixel_format: libc::c_uint,
//     bo_handles: *const *const libc::c_void,
//     strides: *const libc::c_uint,
//     offsets: *const libc::c_uint,
//     flags: libc::c_uint,
// ) -> libc::c_uint {
//     unsafe {
//         let mut buf_id = 0;
//         match crate::ffi::drmModeAddFB2(
//             fd,
//             width,
//             height,
//             pixel_format,
//             bo_handles,
//             strides,
//             offsets,
//             &mut buf_id,
//             flags,
//         ) {
//             r if r == 0 => buf_id,
//             _ => panic!("[DRM] drmModeAddFB2 failed"),
//         }
//     }
// }
