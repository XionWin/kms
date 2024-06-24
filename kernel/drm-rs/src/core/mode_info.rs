#[derive(Debug)]
#[allow(dead_code)]
pub struct ModeInfo {
    handle: *const crate::ffi::models::DrmModeInfo,
    clock: libc::c_uint,

    hdisplay: libc::c_ushort,
    hsync_start: libc::c_ushort,
    hsync_end: libc::c_ushort,
    htotal: libc::c_ushort,
    hskew: libc::c_ushort,
    
    vdisplay: libc::c_ushort,
    vsync_start: libc::c_ushort,
    vsync_end: libc::c_ushort,
    vtotal: libc::c_ushort,
    vscan: libc::c_ushort,
    vrefresh: libc::c_int,

    flags: libc::c_uint,
    mode_type: crate::ffi::enums::DrmModeType,
    name: String,
}

impl ModeInfo {
    pub fn new(mi: &crate::ffi::models::DrmModeInfo) -> Self {
        Self {
            handle: mi,
            clock: mi.clock,
            hdisplay: mi.hdisplay,
            hsync_start: mi.hsync_start,
            hsync_end: mi.hsync_end,
            htotal: mi.htotal,
            hskew: mi.hskew,
            vdisplay: mi.vdisplay,
            vsync_start: mi.vsync_start,
            vsync_end: mi.vsync_end,
            vtotal: mi.vtotal,
            vscan: mi.vscan,
            vrefresh: mi.vrefresh,
            flags: mi.flags,
            mode_type: mi.mode_type,
            name: crate::util::get_string_from_chars(&mi.name),
        }
    }

    pub fn get_mode_type(&self) -> crate::ffi::enums::DrmModeType {
        self.mode_type
    }

    pub fn get_handle(&self) -> *const libc::c_void {
        self.handle as _
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// fn get_name(mi: &crate::ffi::models::DrmModeInfo) -> String {
//     String::from(
//         std::ffi::CStr::from_bytes_until_nul(&mi.name).unwrap().to_str().unwrap()
//     )
// } 

