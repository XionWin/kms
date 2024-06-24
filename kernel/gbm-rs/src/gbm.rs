use std::os::fd::RawFd;

use crate::{Device, Surface, def::{SurfaceFormat, FormatModifier}};

#[derive(Debug)]
pub struct Gbm {
    fd: RawFd,
    surface: Surface,
    surface_format: SurfaceFormat,
    format_modifiers: Vec<FormatModifier>,
    width: libc::c_int,
    height: libc::c_int,
}

impl Gbm {
    pub fn new(fd: RawFd, crtc_width: i32, crtc_height: i32, surface_format: SurfaceFormat, format_modifiers: Vec<FormatModifier>) -> Self
    {
        let surface = Surface::new_with_modifiers(Device::new(fd), crtc_width, crtc_height, surface_format, &format_modifiers);
        Self{
            fd,
            surface,
            surface_format,
            format_modifiers,
            width: crtc_width,
            height: crtc_height,
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.fd
    }
    
    pub fn get_surface(&self) -> &Surface {
        &self.surface
    }
    
    pub fn get_surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    pub fn get_surface_format(&self) -> SurfaceFormat {
        self.surface_format
    }

    pub fn get_format_modifiers(&self) -> &Vec<FormatModifier> {
        &self.format_modifiers
    }

    pub fn get_width(&self) -> libc::c_int {
        self.width
    }

    pub fn get_height(&self) -> libc::c_int {
        self.height
    }

}