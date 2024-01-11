use std::time::SystemTime;
use colored_rs::Colorize;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate colored_rs;

mod utility;


fn main() {
    print_hight_light!("====================[KMS DEMO]====================");
    print_debug!(
        "started_time: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );

    kms_rs::begin_with(init_func, update_fun);
}

fn init_func(context: &kms_rs::Context) {
    gles_rs::viewport(0, 0, context.get_width(), context.get_height());
}

lazy_static! {
    static ref STARTED_TICK: std::time::SystemTime = std::time::SystemTime::now();
}
fn update_fun(_context: &kms_rs::Context) {
    let started_tick = *STARTED_TICK;
    let angle = (std::time::SystemTime::now().duration_since(started_tick).unwrap().as_millis() / 20 % 360) as u32;
    let hsv = color_rs::HSV::new(angle as f32, 1f32, 0.5f32);
    let rgb: color_rs::RGB = hsv.into();
    let (r, g, b) = rgb.into();
    gles_rs::clear_color(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, 1f32);

    gles_rs::clear(gles_rs::GL_COLOR_BUFFER_BIT);
}
