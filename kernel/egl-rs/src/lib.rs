// pub fn test(gbm: gbm::Gbm, func_name: &str) {
//     let func_name = String::from(func_name);
//     let mut func_name = func_name.bytes().collect::<Vec<_>>();
//     func_name.push(b'\0');

//     unsafe {
//         let r: *const fn(libc::c_int, libc::c_int, libc::c_int) -> libc::c_int = eglGetProcAddress(func_name.as_ptr());
//         println!("{:?}", r);

//         let r: extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> libc::c_int = std::mem::transmute(r);
//         let r = r(0x31D7, gbm.get_device().get_handle(), 0);
//         println!("{:?}", r);
//     }

// }

// #[link(name = "EGL")]
// #[allow(improper_ctypes)]
// extern "C" {
//     pub fn eglGetProcAddress(func_name: *const libc::c_char) -> *const fn(libc::c_int, libc::c_int, libc::c_int) -> libc::c_int;
// }
#[macro_use]
extern crate bitflags;

extern crate libc;

mod context;
pub(crate) mod ffi;
pub mod def;


pub use context::*;

pub fn swap_buffers(display: *const libc::c_void, surface: *const libc::c_void) -> bool {
    unsafe { crate::ffi::eglSwapBuffers(display as _, surface as _) }
}

