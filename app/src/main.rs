use std::time::SystemTime;
use colored_rs::Colorize;

#[macro_use]
extern crate colored_rs;

mod utility;


fn main() {
    print_hight_light!("====================[KMS DEMO]====================");
    print_debug!(
        "started_time: {}",
        utility::pretty_print_system_time(SystemTime::now()).green()
    );

    kms_rs::init(Some("/dev/dri/card0"), kms_rs::SurfaceType::OpenGlesV2, init_func, update_fun);
}

fn init_func(context: &kms_rs::Context) {
    nvg_rs::init(&context);
}

fn update_fun(context: &kms_rs::Context) {
    nvg_rs::update(&context);
}

