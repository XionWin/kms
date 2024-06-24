use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BufferObject {
    handle: *const crate::ffi::GbmBufferObject,
}

static mut BUFFER_OBJECT_CACHE: Lazy<HashMap<*const crate::ffi::GbmBufferObject, BufferObject>> =
    Lazy::new(|| HashMap::<*const crate::ffi::GbmBufferObject, BufferObject>::new());

impl BufferObject {
    pub fn get_bo(handle: *const crate::ffi::GbmBufferObject) -> &'static Self {
        unsafe {
            if BUFFER_OBJECT_CACHE.contains_key(&handle) == false {
                let bo = Self { handle };
                BUFFER_OBJECT_CACHE.insert(handle, bo);
                colored_rs::print_debug!("create_bo: {:?}", handle);
            }
            &BUFFER_OBJECT_CACHE[&handle]
        }
    }

    pub(crate) fn get_fb(&self, device: &crate::Device) -> libc::c_uint {
        match unsafe { crate::ffi::gbm_bo_get_user_data(self.handle) } {
            user_data if user_data == std::ptr::null() => {
                let width = unsafe { crate::ffi::gbm_bo_get_width(self.handle) };
                let height = unsafe { crate::ffi::gbm_bo_get_height(self.handle) };
                let pixel_format = unsafe { crate::ffi::gbm_bo_get_format(self.handle) };
                let plane_count = unsafe { crate::ffi::gbm_bo_get_plane_count(self.handle) };

                let mut strides = Vec::<libc::c_uint>::new();
                let mut handles = Vec::<*const libc::c_void>::new();
                let mut offsets = Vec::<libc::c_uint>::new();
                for plane_index in 0..plane_count {
                    strides.push(unsafe {
                        crate::ffi::gbm_bo_get_stride_for_plane(self.handle, plane_index)
                    });
                    handles.push(unsafe {
                        crate::ffi::gbm_bo_get_handle_for_plane(self.handle, plane_index).ptr
                    });
                    offsets
                        .push(unsafe { crate::ffi::gbm_bo_get_offset(self.handle, plane_index) });
                }
                let fb = crate::fb::get_fb2(
                    unsafe { crate::ffi::gbm_device_get_fd(device.get_handle_raw()) },
                    width,
                    height,
                    pixel_format as _,
                    handles.as_ptr(),
                    strides.as_ptr(),
                    offsets.as_ptr(),
                    0,
                );
                unsafe {
                    crate::ffi::gbm_bo_set_user_data(
                        self.handle,
                        fb as _,
                        destroy_user_data_callback,
                    );
                }
                colored_rs::print_debug!("get_fb: {:#x?}", fb);
                fb
            }
            user_data => user_data as _,
        }
    }
}

extern "C" fn destroy_user_data_callback(
    bo: *const crate::ffi::GbmBufferObject,
    data: *const libc::c_void,
) {
    colored_rs::print_debug!("destroy_user_data_callback bo: {:?} data: {:?}", bo, data);
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe {
            if self.handle as u32 == 0 {
                println!("Err");
                return;
            }
            crate::ffi::gbm_bo_destroy(self.handle);
            colored_rs::print_debug!("BufferObject: {:?} droped", self.handle);
        }
    }
}
