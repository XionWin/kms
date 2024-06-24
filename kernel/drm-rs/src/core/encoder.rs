#[derive(Debug)]
#[allow(dead_code)]
pub struct Encoder {
    handle: *const crate::ffi::DrmModeEncoder,
    encoder_id: libc::c_uint,
    encoder_type: crate::ffi::enums::EncoderType,
    crtc_id: libc::c_uint,
    possible_crtcs: libc::c_uint,
    possible_clones: libc::c_uint,
}

impl Encoder {
    pub fn new(e: &crate::ffi::DrmModeEncoder) -> Self {
        Self {
            handle: e,
            encoder_id: e.encoder_id,
            encoder_type: e.encoder_type,
            crtc_id: e.crtc_id,
            possible_crtcs: e.possible_crtcs,
            possible_clones: e.possible_clones,
        }
    }

    pub fn get_encoder_id(&self) -> libc::c_uint {
        self.encoder_id
    }

    pub fn get_crtc_id(&self) -> libc::c_uint {
        self.crtc_id
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeEncoder(self.handle);
            colored_rs::print_debug!("Encoder: {:?} droped", self.handle);
        }
    }
}