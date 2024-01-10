use std::time::SystemTime;

#[macro_use]
extern crate colored_rs;
use colored_rs::Colorize;

#[allow(dead_code)]
mod oflag;
mod utility;

fn main() {
    print_hight_light!("====================[grid-rs]====================");
    print_info!(
        "supported_surface_formats: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );
    print_debug!(
        "datetime: {}",
        utility::pretty_print_system_time(SystemTime::now())
    );

    let default_video_card_info = utility::get_default_video_card_info().unwrap();
    print_info!(
        "default_video_card_path: {:#?}, fd: {:#?}",
        default_video_card_info.path,
        default_video_card_info.fd
    );

    let fd = default_video_card_info.fd;
    let drm = drm_rs::core::Drm::new(fd, |conn| {
        conn.get_connection_status() == drm_rs::ConnectionStatus::Connected
    });
    let mode = drm.get_mode();
    print_info!("actived_mode_name: {:#?}", mode.get_name());

    let (width, height) = (drm.crtc.get_width(), drm.crtc.get_height());

    let mut gbm = gbm_rs::Gbm::new(
        fd,
        width,
        height,
        gbm_rs::def::SurfaceFormat::ARGB8888,
        vec![gbm_rs::def::FormatModifier::DRM_FORMAT_MOD_LINEAR],
    );

    let supported_surface_format = gbm_rs::def::SurfaceFormat::iter()
        .into_iter()
        .filter(|format| {
            gbm.get_surface()
                .get_device()
                .is_format_supported(*format, gbm_rs::def::SurfaceFlags::Linear)
        })
        .collect::<Vec<gbm_rs::def::SurfaceFormat>>();

    print_info!(
        "supported_surface_formats: {}",
        supported_surface_format
            .into_iter()
            .map(|format| format!("{:?} ", format))
            .collect::<Vec<String>>()
            .join(" ")
    );

    let context: egl_rs::Context = egl_rs::Context::new(
        gbm.get_surface().get_handle(),
        gbm.get_surface().get_device().get_handle(),
        width,
        height,
        true,
    );
    print_warning!("context: {:#?}", context);

    initialize(&context, &mut gbm, &drm);
}

pub fn initialize(context: &egl_rs::Context, gbm: &mut gbm_rs::Gbm, drm: &drm_rs::Drm) {
    let surface = gbm.get_surface_mut();

    let display_handle = context.display;
    let surface_handle = context.surface;

    let func = |display: *const libc::c_void, surface: *const libc::c_void| {
        egl_rs::swap_buffers(display, surface)
    };
    surface.register_swap_callback((func, display_handle as _, surface_handle as _));

    let (_, fb) = surface.lock();
    let drm_fd = drm.get_fd();
    let drm_crtc_id = drm.crtc.get_id();
    let drm_connector_ids = &vec![drm.connector.get_id()];
    let drm_mode = drm.get_mode().get_handle();
    match drm_rs::set_crtc(
        drm_fd,
        drm_crtc_id,
        fb as _,
        0,
        0,
        drm_connector_ids.as_ptr(),
        drm_connector_ids.len() as _,
        drm_mode,
    ) {
        result if result == 0 => result,
        _ => panic!("surface initialize set_crtc error"),
    };
}

pub fn update(context: &mut egl_rs::Context, gbm: &mut gbm_rs::Gbm, drm: &drm_rs::Drm) {
    let fd = drm.get_fd();
    let crtc_id = drm.crtc.get_id();

    let surface = gbm.get_surface_mut();
    let (_, fb) = surface.lock();
    if context.vertical_synchronization {
        drm_rs::vertical_synchronization(fd, crtc_id, fb);
    }
}
