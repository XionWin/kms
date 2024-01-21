use nvg_rs::context::Vertex;
use once_cell::sync::Lazy;
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

pub fn init(kms: &mut kms_rs::KMS) -> Graphic<String> {
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

    let vertexes = vec![
        Vertex::new(100.0, 0.0, 1.0, 0.0),
        Vertex::new(200.0, 200.0, 0.0, 1.0),
        Vertex::new(0.0, 200.0, 1.0, 1.0)
    ];
    let indices = vec![
        0, 1, 2
    ];

    
    let mut vao = 0u32;
    gles_rs::gen_vertex_arrays(1, &mut vao);
    gles_rs::bind_vertex_array(vao);
    
    let vbos = gles_rs::gen_buffers(2);
    gles_rs::bind_buffer(gles_rs::def::BufferTarget::ArrayBuffer, vbos[0]);
    gles_rs::buffer_data(
        gles_rs::def::BufferTarget::ArrayBuffer,
        vertexes.as_slice(),
        gles_rs::def::BufferUsageHint::StaticDraw
    );

    gles_rs::bind_buffer(gles_rs::def::BufferTarget::ElementArrayBuffer, vbos[1]);
    gles_rs::buffer_data(
        gles_rs::def::BufferTarget::ElementArrayBuffer,
        indices.as_slice(),
        gles_rs::def::BufferUsageHint::StaticDraw
    );

    let vertex_idx = gles_rs::get_attrib_location(program.get_id(), "aVertex");
    gles_rs::enable_vertex_attrib_array(vertex_idx);
    gles_rs::vertex_attrib_pointer_f32(
        vertex_idx, 
        2, 
        false,
        std::mem::size_of::<Vertex>() as _, 
        0);

        let coord_idx = gles_rs::get_attrib_location(program.get_id(), "aCoord");
        gles_rs::enable_vertex_attrib_array(coord_idx);
        gles_rs::vertex_attrib_pointer_f32(
            coord_idx, 
            2, 
            false,
            std::mem::size_of::<Vertex>() as _, 
            (std::mem::size_of::<f32>() * 2) as _);
  
    gles_rs::uniform2f(&program, "uViewSize", kms.get_width() as _, kms.get_height() as _);
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
    let hsv = nvg_rs::color::Color::hsl(h as _, 1.0, 0.35);
    let (r, g, b, a) = hsv.into();
    gles_rs::clear_color(r, g, b, a);
    gles_rs::clear(gles_rs::ffi::GL_COLOR_BUFFER_BIT | gles_rs::ffi::GL_DEPTH_BUFFER_BIT);

    gles_rs::bind_vertex_array(1);
    gles_rs::draw_arrays(gles_rs::def::BeginMode::Triangles, 0, 3);
}
