bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct DrmModeType: libc::c_int {
        const BUILT_IN = 1<<0;
        const COCK_C = (1<<1 | 1<<0);
        const CRTC_C = (1<<2 | 1<<0);
        const PREFERRED = 1<<3;
        const DEFAULT = 1<<4;
        const USER_DEFAULT = 1<<5;
        const DRIVER = 1<<6;
    }
}