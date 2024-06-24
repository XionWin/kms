#[derive(Debug)]
pub struct Surface {
    pub(crate) handle: *const crate::ffi::GbmSurface,
    device: crate::Device,
    swap_callback: (
        fn(*const libc::c_void, *const libc::c_void) -> bool,
        *const libc::c_void,
        *const libc::c_void,
    ),

    bo_handle: *const crate::ffi::GbmBufferObject,
}

impl Surface {
    pub fn new(
        device: crate::Device,
        width: libc::c_int,
        height: libc::c_int,
        format: crate::def::SurfaceFormat,
        flags: crate::def::SurfaceFlags,
    ) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create(
                    device.get_handle_raw(),
                    width,
                    height,
                    format,
                    flags,
                )
            },
            device,
            swap_callback: (|_, _| true, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }
    pub fn new_with_modifiers(
        device: crate::Device,
        width: libc::c_int,
        height: libc::c_int,
        format: crate::def::SurfaceFormat,
        modifiers: &[crate::def::FormatModifier],
    ) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create_with_modifiers(
                    device.get_handle_raw(),
                    width,
                    height,
                    format,
                    modifiers.as_ptr() as *const _,
                    modifiers.len() as _,
                )
            },
            device,
            swap_callback: (|_, _| true, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }

    pub fn get_device(&self) -> &crate::Device {
        &self.device
    }

    pub fn get_handle(&self) -> *const libc::c_void {
        self.handle as _
    }

    pub fn register_swap_callback(
        &mut self,
        swap_callback: (
            fn(*const libc::c_void, *const libc::c_void) -> bool,
            *const libc::c_void,
            *const libc::c_void,
        ),
    ) {
        self.swap_callback = swap_callback;
    }

    pub fn lock(&mut self) -> (&'static crate::BufferObject, libc::c_uint) {
        self.swap();

        let last_bo_handle = self.bo_handle;
        self.bo_handle = match unsafe { crate::ffi::gbm_surface_lock_front_buffer(self.handle) } {
            handle if handle == std::ptr::null() => panic!("[GBM]: Failed to lock front buffer"),
            handle => handle,
        };
        let bo = crate::BufferObject::get_bo(self.bo_handle);
        let fb = bo.get_fb(&self.device);

        if last_bo_handle != std::ptr::null() {
            unsafe {
                crate::ffi::gbm_surface_release_buffer(self.handle, last_bo_handle);
            }
        }
        (bo, fb)
    }

    fn swap(&self) {
        let (func, param_display, param_surface) = self.swap_callback;
        if !func(param_display, param_surface) {
            panic!("surface swap error");
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::gbm_surface_destroy(self.handle);
            colored_rs::print_debug!("Surface: {:?} droped", self.handle);
        }
    }
}
