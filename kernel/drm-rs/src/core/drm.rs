use std::os::fd::RawFd;

use crate::core::{Connector, Crtc, Encoder};

#[derive(Debug)]
pub struct Drm {
    pub fd: RawFd,
    pub handle: *const crate::ffi::DrmModeRes,
    pub connector: Connector,
    pub encoder: Encoder,
    pub crtc: Crtc,
}

impl Drm {
    pub fn new<T>(fd: libc::c_int, connector_selector: T) -> Self
    where 
        T: FnMut(&Connector) -> bool,
    {
        let handle: *const crate::ffi::DrmModeRes = unsafe {
            crate::ffi::drmModeGetResources(fd)
        };
        let resources = unsafe {
            handle.as_ref().unwrap()
        };

        let mut connectors = Self::get_connectors(fd, resources);
        let connector = match connectors.iter().position(connector_selector) {
            Some(index) => connectors.remove(index),
            None => panic!("Connector not found")
        };
        
        let mut encoders = Self::get_encoders(fd, resources);
        let encoder = match encoders.iter().position(|x| x.get_encoder_id() == connector.get_encoder_id()) {
            Some(index) => encoders.remove(index),
            None => panic!("Encoder not found")
        };

        let mut crtcs = Self::get_crtcs(fd, resources);
        let crtc = match crtcs.iter().position(|x| x.get_id() == encoder.get_crtc_id()) {
            Some(index) => crtcs.remove(index),
            None => panic!("Crtc not found")
        };
        
        Self{
            fd,
            handle,
            connector,
            encoder,
            crtc,
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.fd
    }
    
    pub fn get_mode(&self) -> Option<&crate::core::ModeInfo> {
        match &self.connector.modes {
            Some(modes) => modes.iter().find(|x| crate::bitwise_contains!(x.get_mode_type(), crate::ffi::enums::DrmModeType::PREFERRED)),
            None => panic!("Mode not found")
        }
    }

    fn get_crtcs(fd: libc::c_int, r: &crate::ffi::DrmModeRes) -> Vec<crate::core::Crtc> {
        unsafe {
            std::slice::from_raw_parts(r.crtcs, r.count_crtcs as usize).iter().map(|x| {
                crate::core::Crtc::new(crate::ffi::drmModeGetCrtc(fd, *x).as_ref().unwrap())
            }).collect::<Vec<_>>()
        }
    }
    
    fn  get_connectors(fd: libc::c_int, r: &crate::ffi::DrmModeRes) -> Vec<crate::core::Connector> {
        unsafe {
            std::slice::from_raw_parts(r.connectors, r.count_connectors as usize).iter().map(|x| {
                crate::core::Connector::new(crate::ffi::drmModeGetConnector(fd, *x).as_ref().unwrap())
            }).collect::<Vec<_>>()
        }
    }

    fn get_encoders(fd: libc::c_int, r: &crate::ffi::DrmModeRes) -> Vec<crate::core::Encoder> {
        unsafe {
            std::slice::from_raw_parts(r.encoders, r.count_encoders as usize).iter().map(|x| {
                crate::core::Encoder::new(crate::ffi::drmModeGetEncoder(fd, *x).as_ref().unwrap())
            }).collect::<Vec<_>>()
        }
    }

    pub fn get_fbs(fd: libc::c_int, r: &crate::ffi::DrmModeRes) -> Vec<crate::core::Framebuffer> {
        unsafe {
            std::slice::from_raw_parts(r.fbs, r.count_fbs as usize).iter().map(|x| {
                crate::core::Framebuffer::new( crate::ffi::drmModeGetFB(fd, *x).as_ref().unwrap())
            }).collect::<Vec<_>>()
        }
    }

}

impl Drop for Drm {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeResources(self.handle);
            colored_rs::print_debug!("Drm: {:?} droped", self.handle);
        }
    }
}

pub fn is_validated_handle(fd: libc::c_int) -> bool {
    let handle: *const crate::ffi::DrmModeRes = unsafe {
        crate::ffi::drmModeGetResources(fd)
    };
    handle != std::ptr::null()
}