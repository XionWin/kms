use std::time::SystemTime;

#[macro_use]
extern crate colored_rs;
use colored_rs::Colorize;

use crate::egl_context_outside_init::EglContextOutsideInitTrait;

#[allow(dead_code)]
mod oflag;
mod utility;
mod egl_context_outside_init;

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

    context.initialize(&mut gbm, &drm);

    
    gles_rs::viewport(0, 0, width, height);
    let first_tick = std::time::SystemTime::now();
    loop {
        // gles_rs::clear_color(1f32, 1f32, 0f32, 0.3f32);
        let angle = (std::time::SystemTime::now().duration_since(first_tick).unwrap().as_millis() / 20 % 360) as u32;
        let hsv = color_rs::HSV::new(angle as f32, 1.0f32, 0.5f32);
        let rgb: color_rs::RGB = hsv.into();
        let (r, g, b) = rgb.into();
        gles_rs::clear_color(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, 1f32);

        gles_rs::clear(0x00004000);
        context.frame_vertical_synchronize(&mut gbm, &drm);
    }
}

