#[repr(C)]
#[derive(Debug)]
pub struct DrmModeInfo {
    pub clock: libc::c_uint,

    pub hdisplay: libc::c_ushort,
    pub hsync_start: libc::c_ushort,
    pub hsync_end: libc::c_ushort,
    pub htotal: libc::c_ushort,
    pub hskew: libc::c_ushort,
    pub vdisplay: libc::c_ushort,
    pub vsync_start: libc::c_ushort,
    pub vsync_end: libc::c_ushort,
    pub vtotal: libc::c_ushort,
    pub vscan: libc::c_ushort,

    pub vrefresh: libc::c_int,

    pub flags: libc::c_uint,
    pub mode_type: crate::ffi::enums::DrmModeType,
    pub name: [libc::c_char; 32usize]
}