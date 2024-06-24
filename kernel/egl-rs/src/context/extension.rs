use crate::{ffi::{EGLNativeWindowType, EglConfig, EglContext, EglDisplay, EglSurface}, def::SurfaceType};
use std::{ffi::CStr, vec};

const EGL_PLATFORM_GBM_KHR: libc::c_uint = 0x31D7;

pub(crate) fn get_display(device_handle: *const libc::c_void, extensions: &str) -> EglDisplay {

    let display = match extensions.contains("EGL_EXT_platform_base") {
        true => {
            get_egl_get_platform_display_ext_func("eglGetPlatformDisplayEXT")(
                EGL_PLATFORM_GBM_KHR,
                device_handle,
                std::ptr::null(),
            )
        },
        _ => panic!("Can't get \"EGL_EXT_platform_base\" in extensions")
    };

    match display {
        display if display.is_null() => panic!("[EGL] GetDisplay failed.: {:?}", unsafe {
            crate::ffi::eglGetError()
        }),
        display => display,
    }
}

pub(crate) fn egl_initialize(display: EglDisplay) -> (libc::c_int, libc::c_int) {
    let (mut major, mut minor) = (0i32, 0i32);
    if unsafe { crate::ffi::eglInitialize(display, &mut major, &mut minor) } {
        (major, minor)
    }
    else {
        panic!(
            "[EGL] Failed to initialize EGL display. Error code: {:?}",
            unsafe { crate::ffi::eglGetError() }
        );
    }
}

pub(crate) fn get_config(display: EglDisplay, surface_type: SurfaceType) -> EglConfig {
    match get_extensions_by_display(display) {
        Some(extensions) if !extensions.contains("EGL_EXT_image_dma_buf_import_modifiers") => {
            panic!("Can't get \"EGL_EXT_image_dma_buf_import_modifiers\" in extensions")
        }
        None => panic!("Get \"eglGetPlatformDisplayEXT\" error"),
        _ => {}
    };
    egl_bind_api(crate::def::RenderAPI::GLES);
    let desired_config = [
        crate::def::Definition::SURFACE_TYPE,
        surface_type.get_definition(),
        crate::def::Definition::RENDERABLE_TYPE,
        surface_type.get_definition(),
        crate::def::Definition::RED_SIZE,
        8,
        crate::def::Definition::GREEN_SIZE,
        8,
        crate::def::Definition::BLUE_SIZE,
        8,
        crate::def::Definition::ALPHA_SIZE,
        8,
        crate::def::Definition::STENCIL_SIZE,
        8,
        crate::def::Definition::SAMPLE_BUFFERS,
        0,
        crate::def::Definition::SAMPLES,
        0,
        crate::def::Definition::NONE,
    ];
    let size = get_egl_config_count(display, (&desired_config).as_ptr() as _);
    let configs = get_egl_configs(display, &desired_config as _, size as _);
    configs[0]
}

pub(crate) fn get_context(
    display: EglDisplay,
    config: EglConfig,
    attrib_list: *const libc::c_int,
) -> EglContext {
    get_egl_context(display, config, attrib_list)
}

pub(crate) fn get_surface(display: EglDisplay, config: EglConfig, surface_handle: *const libc::c_void) -> EglSurface {
    get_egl_surface(display, config, surface_handle as _)
}

pub(crate) fn egl_make_current(display: EglDisplay, surface: EglSurface, context: EglContext) {
    if !unsafe { crate::ffi::eglMakeCurrent(display, surface, surface, context) } {
        panic!("[EGL] Failed to make current, error {:?}", unsafe {
            crate::ffi::eglGetError()
        });
    }
}

pub(crate) fn get_egl_surface(
    display: EglDisplay,
    config: EglConfig,
    native_wnd_handle: EGLNativeWindowType,
) -> EglSurface {
    match unsafe { crate::ffi::eglCreateWindowSurface(display, config, native_wnd_handle, 0 as _) }
    {
        handle if handle.is_null() => {
            panic!("[EGL] Failed to create egl surface, error {:?}", unsafe {
                crate::ffi::eglGetError()
            })
        }
        handle => handle,
    }
}

pub(crate) fn get_egl_context(
    display: EglDisplay,
    config: EglConfig,
    attrib_list: *const libc::c_int,
) -> EglContext {
    match unsafe { crate::ffi::eglCreateContext(display, config, 0 as _, attrib_list) } {
        handle if handle.is_null() => {
            panic!("[EGL] Failed to create egl context, error {:?}", unsafe {
                crate::ffi::eglGetError()
            })
        }
        handle => handle,
    }
}

pub(crate) fn get_egl_config_count(
    display: EglDisplay,
    desired_config: *const libc::c_int,
) -> libc::c_uint {
    let mut num_configs = 0;

    match unsafe {
        crate::ffi::eglChooseConfig(
            display,
            desired_config,
            std::ptr::null(),
            0,
            &mut num_configs,
        )
    } {
        true if num_configs == 0 => {
            panic!("No matched eglConfig");
        }
        false => {
            panic!("eglChooseConfig error");
        }
        _ => { /* expected, do nothing */ }
    };
    num_configs as _
}

pub(crate) fn get_egl_configs(
    display: EglDisplay,
    desired_config: *const libc::c_int,
    count: libc::c_int,
) -> Vec<EglConfig> {
    let mut num_configs = 0;

    let configs = vec![std::ptr::null(), count as _];
    match unsafe {
        crate::ffi::eglChooseConfig(
            display,
            desired_config,
            configs.as_ptr() as _,
            configs.len() as _,
            &mut num_configs,
        )
    } {
        true if num_configs < 1 => {
            panic!("No matched eglConfig");
        }
        false => {
            panic!("eglChooseConfig error");
        }
        _ => configs,
    }
}

pub(crate) fn egl_bind_api(render_api: crate::def::RenderAPI) {
    if !unsafe { crate::ffi::eglBindAPI(render_api) } {
        panic!("[EGL] Failed to bind EGL Api: {:?}", unsafe {
            crate::ffi::eglGetError()
        });
    }
}

pub(crate) fn get_egl_get_platform_display_ext_func(
    func_name: &str,
) -> extern "C" fn(libc::c_uint, *const libc::c_void, *const libc::c_uint) -> EglDisplay {
    let mut func_name = String::from(func_name)
        .bytes()
        .collect::<Vec<_>>();
    func_name.push(b'\0');

    unsafe { std::mem::transmute(crate::ffi::eglGetProcAddress(func_name.as_ptr())) }
}

pub(crate) fn get_version_by_display(display: EglDisplay) -> Option<String> {
    unsafe {
        get_string_from_ptr(crate::ffi::eglQueryString(
            display,
            crate::def::Definition::VERSION,
        ))
    }
}

pub(crate) fn get_vendor_by_display(display: EglDisplay) -> Option<String> {
    unsafe {
        get_string_from_ptr(crate::ffi::eglQueryString(
            display,
            crate::def::Definition::VENDOR,
        ))
    }
}

pub(crate) fn get_extensions_by_display(display: EglDisplay) -> Option<String> {
    unsafe {
        get_string_from_ptr(crate::ffi::eglQueryString(
            display,
            crate::def::Definition::EXTENSIONS,
        ))
    }
}

pub(crate) fn get_string_from_ptr(ptr: *const libc::c_char) -> Option<String> {
    match ptr {
        ptr if ptr != std::ptr::null() => Some(String::from(
            unsafe { CStr::from_ptr(ptr) }.to_str().unwrap(),
        )),
        _ => None,
    }
}
