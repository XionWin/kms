use std::time::SystemTime;

extern crate drm_rs;
extern crate gbm_rs;
extern crate libc;

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
        "default_video_card path: {:#?}, fd: {:#?}",
        default_video_card_info.path,
        default_video_card_info.fd
    );

    let fd = default_video_card_info.fd;
    let drm = drm_rs::core::Drm::new(fd, |conn| {
        conn.get_connection_status() == drm_rs::ConnectionStatus::Connected
    });

    let (width, height) = (drm.crtc.get_width(), drm.crtc.get_height());
    let mode = drm.get_mode();
    print_warning!("mode: {:#?}", mode);

    let gbm = gbm_rs::Gbm::new(
        fd,
        width,
        height,
        gbm_rs::def::SurfaceFormat::ARGB8888,
        vec![gbm_rs::def::FormatModifier::DRM_FORMAT_MOD_LINEAR],
    );
    // print_warning!("gbm: {:#?}", gbm);

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

    // let mut _context: egl_rs::Context = egl_rs::Context::new(
    //     gbm.get_surface().get_handle(),
    //     gbm.get_surface().get_device().get_handle(),
    //     width,
    //     height,
    //     true,
    // );
}
