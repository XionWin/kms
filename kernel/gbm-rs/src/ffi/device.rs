use std::os::fd::RawFd;

#[repr(C)]
pub struct GbmDevice {}

#[link(name = "gbm")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn gbm_create_device(fd: RawFd) -> *const GbmDevice;
    pub fn gbm_device_get_fd(device: *const GbmDevice) -> RawFd;
    pub fn gbm_device_destroy(handle: *const GbmDevice);
    pub fn gbm_device_is_format_supported(handle: *const GbmDevice, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> bool;
}