use std::time::SystemTime;
use colored_rs::Colorize;

#[macro_use]
extern crate colored_rs;
#[macro_use]
extern crate kms_rs;

mod utility;


fn main() {
    print_hight_light!("====================[KMS DEMO]====================");
    print_debug!(
        "started_time: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );

    let mut kms = kms_rs::KMS::new(Some("/dev/dri/card0"), kms_rs::SurfaceType::OpenGlesV2);
    begin_render!(init, update, &mut kms);
}


pub fn init(kms: &mut kms_rs::KMS) -> nvg_rs::Graphic {
    colored_rs::print_debug!("gl_extensions: {:?}", gles_rs::get_string(gles_rs::StringName::Extensions));
    colored_rs::print_debug!("gl_version: {:?}", gles_rs::get_string(gles_rs::StringName::Version));
    colored_rs::print_debug!("gl_sharding Language Version: {:?}", gles_rs::get_string(gles_rs::StringName::ShadingLanguageVersion));
    colored_rs::print_debug!("gl_vendor: {:?}", gles_rs::get_string(gles_rs::StringName::Vendor));
    colored_rs::print_debug!("gl_renderer: {:?}", gles_rs::get_string(gles_rs::StringName::Renderer));
    kms.init_double_buffer();
    let width = kms.get_width();
    let height = kms.get_height();
    let graphic = nvg_rs::Graphic::new(width, height);
    nvg_rs::init(&graphic);
    graphic
}

pub fn update(kms: &mut kms_rs::KMS, graphic: &mut nvg_rs::Graphic) {
    nvg_rs::update(graphic);
    kms.wait_vertical_synchronize();
}


