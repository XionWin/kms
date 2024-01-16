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

    // nvg_rs::test_func();

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
    let graphic = Graphic::new(kms.get_width(), kms.get_height(), String::from("tag"));

    let program = gles_rs::GfxProgram::new(
        "resources/shaders/nvgv2.vert",
        "resources/shaders/nvgv2.frag",
    );
    program.active();
    gles_rs::viewport(0, 0, graphic.get_width(), graphic.get_height());
    graphic
}

static STARTED_TICK: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
pub fn update(_kms: &mut kms_rs::KMS, _graphic: &mut Graphic<String>) {
    let started_tick = STARTED_TICK.to_owned();
    let angle = std::time::SystemTime::now()
        .duration_since(started_tick)
        .unwrap()
        .as_millis() as f64 / 3000f64 % 1f64;
    let hsv = color_rs::HSV::new(angle as f32, 1f32, 0.75f32);
    let rgb: color_rs::RGB = hsv.into();
    let (r, g, b) = rgb.into();
    gles_rs::clear_color(
        r as f32 / 255f32,
        g as f32 / 255f32,
        b as f32 / 255f32,
        1f32,
    );
    gles_rs::clear(gles_rs::GL_COLOR_BUFFER_BIT);
}
