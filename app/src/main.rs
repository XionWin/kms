use once_cell::sync::Lazy;
use std::time::SystemTime;

#[macro_use]
extern crate colored_rs;
#[macro_use]
extern crate kms_rs;

use colored_rs::Colorize;
use kms_rs::Graphic;
mod utility;

fn main() {
    print_hight_light!("====================[KMS DEMO]====================");
    print_debug!(
        "started_time: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );

    nvg_rs::test_func();

    let mut kms = kms_rs::KMS::new(Some("/dev/dri/card0"), kms_rs::SurfaceType::OpenGlesV2);
    begin_render!(init, update, &mut kms);
}

pub fn init(kms: &mut kms_rs::KMS) -> Graphic<String> {
    colored_rs::print_debug!(
        "gl_extensions: {:?}",
        gles_rs::get_string(gles_rs::StringName::Extensions)
    );
    colored_rs::print_debug!(
        "gl_version: {:?}",
        gles_rs::get_string(gles_rs::StringName::Version)
    );
    colored_rs::print_debug!(
        "gl_sharding_language_version: {:?}",
        gles_rs::get_string(gles_rs::StringName::ShadingLanguageVersion)
    );
    colored_rs::print_debug!(
        "gl_vendor: {:?}",
        gles_rs::get_string(gles_rs::StringName::Vendor)
    );
    colored_rs::print_debug!(
        "gl_renderer: {:?}",
        gles_rs::get_string(gles_rs::StringName::Renderer)
    );

    gles_rs::GfxProgram::new(
        "resources/shaders/nvgv2.vert",
        "resources/shaders/nvgv2.frag",
    )
    .active();

    gles_rs::viewport(0, 0, kms.get_width(), kms.get_height());
    Graphic::new(kms.get_width(), kms.get_height(), String::from("tag"))
}

static STARTED_TICK: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
pub fn update(_kms: &mut kms_rs::KMS, _graphic: &mut Graphic<String>) {
    let started_tick = STARTED_TICK.to_owned();
    let h = std::time::SystemTime::now()
        .duration_since(started_tick)
        .unwrap()
        .as_millis() as f64
        / 3000f64
        % 1f64;
    let hsv = nvg_rs::Color::hsl(h as _, 1.0, 0.35);
    let (r, g, b, a) = hsv.into();
    gles_rs::clear_color(r, g, b, a);
    gles_rs::clear(gles_rs::GL_COLOR_BUFFER_BIT);
}
