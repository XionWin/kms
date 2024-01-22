use gles_rs::GfxProgram;
use std::time::SystemTime;

#[macro_use]
extern crate colored_rs;
#[macro_use]
extern crate kms_rs;

use colored_rs::Colorize;
use kms_rs::Graphic;
mod utility;
mod renderer;

fn main() {
    print_hight_light!("====================[KMS DEMO]====================");
    print_debug!(
        "started_time: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );

    let mut kms = kms_rs::KMS::new(None, kms_rs::SurfaceType::OpenGlesV2);
    begin_render!(init, update, &mut kms);
}

pub fn init(kms: &mut kms_rs::KMS) -> Graphic<GfxProgram> {
    colored_rs::print_debug!(
        "gl_extensions: {:?}",
        gles_rs::get_string(gles_rs::def::StringName::Extensions)
    );
    colored_rs::print_debug!(
        "gl_version: {:?}",
        gles_rs::get_string(gles_rs::def::StringName::Version)
    );
    colored_rs::print_debug!(
        "gl_sharding_language_version: {:?}",
        gles_rs::get_string(gles_rs::def::StringName::ShadingLanguageVersion)
    );
    colored_rs::print_debug!(
        "gl_vendor: {:?}",
        gles_rs::get_string(gles_rs::def::StringName::Vendor)
    );
    colored_rs::print_debug!(
        "gl_renderer: {:?}",
        gles_rs::get_string(gles_rs::def::StringName::Renderer)
    );

    let program = gles_rs::GfxProgram::new(
        "resources/shaders/nvgv2.vert",
        "resources/shaders/nvgv2.frag",
    );
    program.active();

    gles_rs::uniform2f(gles_rs::get_uniform_location(&program, "uViewSize"), kms.get_width() as _, kms.get_height() as _);
    gles_rs::viewport(0, 0, kms.get_width(), kms.get_height());
    
    let (r, g, b, a) = nvg_rs::color::Color::rgb_i(25, 25, 112).into();
    gles_rs::clear_color(r, g, b, a);

    let mut graphic = Graphic::new(kms.get_width(), kms.get_height(), program);
    renderer::init(&mut graphic);
    graphic
}

pub fn update(_kms: &mut kms_rs::KMS, graphic: &mut Graphic<GfxProgram>) {
    renderer::update(graphic);
}
