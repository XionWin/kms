use gles_rs::GfxProgram;
use kms_rs::Graphic;
use nvg_rs::context::Vertex;
// use once_cell::sync::Lazy;

pub fn init(graphic: &mut Graphic<GfxProgram>) {
    let (width, height) = (graphic.get_width() as f32, graphic.get_height() as f32);
    let program = graphic.get_tag_mut();
    
    let image = image::ImageReader::open("resources/images/items.png").unwrap().decode().unwrap();
    let (image_width, image_height) = (image.width() as f32, image.height() as f32);
    let (x, y, w, h) = ((width - image_width) / 2.0, (height - image_height) / 2.0, image_width, image_height);

    let vertexes = vec![
        Vertex::new(x, y, 0.0, 0.0),
        Vertex::new(x + w, y + h, 1.0, 1.0),
        Vertex::new(x, y + h, 0.0, 1.0),
        Vertex::new(x + w, y, 1.0, 0.0)
    ];
    let indices: Vec<u32> = vec![
        0, 1, 2,
        0, 3, 1
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

   
    
    gles_rs::uniform_1i(gles_rs::get_uniform_location(program.get_id(), "uTexture"), 0);
    let texture = gles_rs::GfxTexture::new(gles_rs::def::TextureUnit::Texture0, gles_rs::def::TextureMinFilter::Nearest);
    
    let image_data = image.to_rgba8().into_vec();
    let image_data = gles_rs::ImageData {
        width: image_width as _,
        height: image_height as _,
        value: image_data
    };
    texture.load(&image_data);
    program.add_texture(texture);
}


// static STARTED_TICK: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
pub fn update(graphic: &mut Graphic<GfxProgram>) {
    // let started_tick = STARTED_TICK.to_owned();
    // let h = std::time::SystemTime::now()
    //     .duration_since(started_tick)
    //     .unwrap()
    //     .as_millis() as f64
    //     / 10_000f64
    //     % 1f64;
    // let hsv = nvg_rs::color::Color::hsl(h as _, 1.0, 0.35);
    // let (r, g, b, a) = hsv.into();
    // gles_rs::clear_color(r, g, b, a);

    gles_rs::clear(gles_rs::ffi::GL_COLOR_BUFFER_BIT | gles_rs::ffi::GL_DEPTH_BUFFER_BIT);
    gles_rs::bind_vertex_array(1);
    // Enable Alpha
    gles_rs::enable(gles_rs::def::EnableCap::Blend);
    gles_rs::blend_func(gles_rs::def::BlendingFactor::SrcAlpha, gles_rs::def::BlendingFactor::OneMinusSrcAlpha);
    
    gles_rs::uniform_1i(gles_rs::get_uniform_location(graphic.get_tag().get_id(), "uTexture"), 0);

    gles_rs::draw_elements::<u32>(gles_rs::def::BeginMode::Triangles, 6, gles_rs::def::DrawElementsType::UnsignedInt, None);
}