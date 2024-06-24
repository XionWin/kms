use crate::{ffi::EglDisplay, def::SurfaceType};

use super::extension::*;

#[derive(Debug)]
pub struct Context {
    pub display: crate::ffi::EglDisplay,
    pub config: crate::ffi::EglConfig,
    pub context: crate::ffi::EglContext,
    pub surface: crate::ffi::EglSurface,
    pub version: (libc::c_int, libc::c_int),
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub is_vertical_synchronize: bool,
}

#[allow(dead_code)]
impl Context {
    pub fn new(surface_handle: *const libc::c_void, device_handle: *const libc::c_void, surface_type: SurfaceType, width: libc::c_int, height: libc::c_int, vertical_synchronization: bool) -> Self {
        print_debug_display_info(std::ptr::null());
        let extensions = get_extensions_by_display(std::ptr::null()).expect("Get client extensions error");

        let display = get_display(device_handle, &extensions);
        let (major, minor) = egl_initialize(display);
        let config = get_config(display, surface_type);
        print_debug_display_info(display);

        let context_attrib = [
            crate::def::Definition::CONTEXT_CLIENT_VERSION,
            surface_type.get_version(),
            crate::def::Definition::NONE,
        ];
        let context = get_context(display, config, &context_attrib as _);
        let surface = get_surface(display, config, surface_handle);

        egl_make_current(display, surface, context);

        Self {
            width,
            height,
            display,
            config,
            context,
            surface,
            version: (major, minor),
            is_vertical_synchronize: vertical_synchronization,
        }
    }

    pub fn get_width(&self) -> libc::c_int {
        self.width
    }

    pub fn get_height(&self) -> libc::c_int {
        self.height
    }

    // pub fn initialize(&mut self, drm: drm_rs::Drm) {
    //     let surface = self.gbm.get_surface_mut();

    //     let display_handle = self.display;
    //     let surface_handle = self.surface;

    //     let func = |display: *const libc::c_void, surface: *const libc::c_void| {
    //         unsafe {
    //             crate::ffi::eglSwapBuffers(display as _, surface as _)
    //          }
    //     };
    //     surface.register_swap_callback((func, display_handle as _, surface_handle as _));

        
    //     let (_, fb) = surface.lock();
    //     let drm_fd = drm.get_fd();
    //     let drm_crtc_id = drm.crtc.get_id();
    //     let drm_connector_ids = &vec![drm.connector.get_id()];
    //     let drm_mode = drm.get_mode().get_handle();
    //     match drm_rs::set_crtc(drm_fd, drm_crtc_id, fb as _, 0, 0, drm_connector_ids.as_ptr(), drm_connector_ids.len() as _, drm_mode) {
    //         result if result == 0 => result,
    //         _ => panic!("surface initialize set_crtc error")
    //     };
    // }

    // pub fn update(&mut self, drm: drm_rs::Drm) {
    //     let fd = drm.get_fd();
    //     let crtc_id = drm.crtc.get_id();

    //     let surface = self.gbm.get_surface_mut();
    //     let (_, fb) = surface.lock();
    //     if self.vertical_synchronization {
    //         vertical_synchronization(fd, crtc_id, fb);
    //     }
    // }

}

// fn vertical_synchronization(fd: RawFd, crtc_id: libc::c_uint, fb: libc::c_uint) {
//     let evt_context = drm_rs::models::EventContext {
//         version: DRM_CONTEXT_VERSION,
//         vblank_handler,
//         page_flip_handler,
//     };

//     let mut user_data = 1;
//     match drm_rs::page_flip( fd, crtc_id, fb as _, drm_rs::enums::PageFlipFlags::FLIP_EVENT, &mut user_data as *mut libc::c_int as _) {
//         result if result != 0 => panic!("page_flip error"),
//         _ => {}
//     }

//     while user_data != 0 {
//         let r = drm_rs::handle_event(fd, &evt_context as *const _ as _);
//         if r != 0 {
//             panic!("handle_event result: {:?}", r);
//         }
//     }
// }

// const DRM_CONTEXT_VERSION: libc::c_int = 2;
// extern fn vblank_handler(
//     _fd: libc::c_int,
//     _sequence: libc::c_uint,
//     _tv_sec: libc::c_uint,
//     _tv_usec: libc::c_uint,
//     _user_data: *mut libc::c_void,
// ) {}
// extern fn page_flip_handler(
//     _fd: libc::c_int,
//     _sequence: libc::c_uint,
//     _tv_sec: libc::c_uint,
//     _tv_usec: libc::c_uint,
//     user_data: *mut libc::c_void,
// ) {
//     unsafe {
//         *(user_data as *mut libc::c_int) = 0;
//     }
// }


fn print_debug_display_info(display: EglDisplay) {
    let name = if display == std::ptr::null() {"client"} else {"display"};
    
    let version = get_version_by_display(display);
    colored_rs::print_debug!("{name} version: {version:?}");
    let vendor = get_vendor_by_display(display);
    colored_rs::print_debug!("{name} vendor: {vendor:?}");
    let extensions = get_extensions_by_display(display);
    colored_rs::print_debug!("{name} extensions: {extensions:?}");
}



