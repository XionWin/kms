#[macro_use]
extern crate anyhow;

use libc::c_uint;
use nvg_rs::{
    color::Color,
    context::{BlendFactor, CompositeOperationState, ImageFlags, ImageId, Paint, Path, Vertex},
    math::{Bounds, Extent, Transform},
    renderer::{self, *},
};
use slab::Slab;

#[allow(dead_code)]
struct Shader {
    prog: c_uint,
    frag: c_uint,
    vert: c_uint,
    loc_viewsize: i32,
    loc_tex: i32,
    // loc_frag: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        opengl_rs::delete_program(self.prog);
        opengl_rs::delete_shader(self.vert);
        opengl_rs::delete_shader(self.frag);
    }
}

impl Shader {
    unsafe fn load() -> anyhow::Result<Shader> {
        let prog = opengl_rs::create_program();
        let vert = opengl_rs::create_shader(opengl_rs::def::ShaderType::VertexShader);
        let frag = opengl_rs::create_shader(opengl_rs::def::ShaderType::FragmentShader);
        let vert_source =
            std::ffi::CString::from_vec_unchecked(include_bytes!("shader.vert").to_vec());
        let frag_source =
            std::ffi::CString::from_vec_unchecked(include_bytes!("shader.frag").to_vec());

        opengl_rs::shader_source(vert, &(vert_source.to_str().unwrap()));
        opengl_rs::shader_source(frag, &(frag_source.to_str().unwrap()));

        opengl_rs::compile_shader(vert);
        let status = opengl_rs::get_shaderiv(vert);
        if status != gl::TRUE as i32 {
            return Err(shader_error(vert, "shader.vert"));
        }

        opengl_rs::compile_shader(frag);
        let status = opengl_rs::get_shaderiv(frag);
        if status != gl::TRUE as i32 {
            return Err(shader_error(vert, "shader.frag"));
        }

        opengl_rs::attach_shader(prog, vert);
        opengl_rs::attach_shader(prog, frag);

        opengl_rs::bind_attrib_location(prog, 0, "vertex");
        opengl_rs::bind_attrib_location(prog, 1, "tcoord");

        opengl_rs::link_program(prog);
        opengl_rs::check_link_status(prog);

        Ok(Shader {
            prog,
            frag,
            vert,
            loc_viewsize: opengl_rs::get_uniform_location(prog, "viewSize"),
            loc_tex: opengl_rs::get_uniform_location(prog, "tex"),
            // loc_frag: gl::GetUniformBlockIndex(prog, "frag"),
        })
    }
}

#[allow(dead_code)]
enum ShaderType {
    FillGradient,
    FillImage,
    Simple,
    Image,
}

#[derive(PartialEq, Eq)]
enum CallType {
    Fill,
    ConvexFill,
    Stroke,
    Triangles,
}

#[allow(dead_code)]
struct Blend {
    src_rgb: gl::types::GLenum,
    dst_rgb: gl::types::GLenum,
    src_alpha: gl::types::GLenum,
    dst_alpha: gl::types::GLenum,
}

impl From<CompositeOperationState> for Blend {
    fn from(state: CompositeOperationState) -> Self {
        Blend {
            src_rgb: convert_blend_factor(state.src_rgb),
            dst_rgb: convert_blend_factor(state.dst_rgb),
            src_alpha: convert_blend_factor(state.src_alpha),
            dst_alpha: convert_blend_factor(state.dst_alpha),
        }
    }
}

#[allow(dead_code)]
struct Call {
    call_type: CallType,
    image: Option<usize>,
    path_offset: usize,
    path_count: usize,
    triangle_offset: usize,
    triangle_count: usize,
    uniform_offset: usize,
    blend_func: Blend,
}

struct Texture {
    tex: gl::types::GLuint,
    width: usize,
    height: usize,
    texture_type: TextureType,
    flags: ImageFlags,
}

impl Drop for Texture {
    fn drop(&mut self) {
        opengl_rs::delete_texture(self.tex)
    }
}

struct GLPath {
    fill_offset: usize,
    fill_count: usize,
    stroke_offset: usize,
    stroke_count: usize,
}

#[derive(Default)]
#[allow(dead_code)]
struct FragUniforms {
    scissor_mat: [f32; 12],
    paint_mat: [f32; 12],
    inner_color: Color,
    outer_color: Color,
    scissor_ext: [f32; 2],
    scissor_scale: [f32; 2],
    extent: [f32; 2],
    radius: f32,
    feather: f32,
    stroke_mult: f32,
    stroke_thr: f32,
    tex_type: i32,
    type_: i32,
}

#[allow(dead_code)]
pub struct Renderer {
    shader: Shader,
    textures: Slab<Texture>,
    view: Extent,
    vert_buf: gl::types::GLuint,
    vert_arr: gl::types::GLuint,
    frag_buf: gl::types::GLuint,
    calls: Vec<Call>,
    paths: Vec<GLPath>,
    vertexes: Vec<Vertex>,
    uniforms: Vec<u8>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        opengl_rs::delete_buffer(self.frag_buf);
        opengl_rs::delete_buffer(self.vert_buf);
        // gl::DeleteVertexArrays(1, &self.vert_arr);
    }
}

#[allow(dead_code)]
impl Renderer {
    pub fn create() -> anyhow::Result<Renderer> {
        unsafe {
            let shader = Shader::load()?;

            let vert_arr = opengl_rs::gen_vertex_array();

            let vert_buf = opengl_rs::gen_buffer();

            // gl::UniformBlockBinding(shader.prog, shader.loc_frag, 0);
            let frag_buf = opengl_rs::gen_buffer();

            // let align: usize = std::mem::zeroed();
            // gl::GetIntegerv(opengl_rs::ffi::GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT, &mut align);

            opengl_rs::finish();

            Ok(Renderer {
                shader,
                textures: Default::default(),
                view: Default::default(),
                vert_buf,
                vert_arr,
                frag_buf,
                calls: Default::default(),
                paths: Default::default(),
                vertexes: Default::default(),
                uniforms: Default::default(),
            })
        }
    }

    #[allow(unused_variables)]
    unsafe fn set_uniforms(&self, offset: usize, img: Option<usize>) {
        // gl::BindBufferRange(
        //     gl::UNIFORM_BUFFER,
        //     0,
        //     self.frag_buf,
        //     (offset * self.frag_size) as isize,
        //     std::mem::size_of::<FragUniforms>() as isize,
        // );

        if let Some(img) = img {
            if let Some(texture) = self.textures.get(img) {
                opengl_rs::bind_texture(opengl_rs::def::TextureTarget::Texture2D, texture.tex);
            }
        } else {
            opengl_rs::bind_texture(opengl_rs::def::TextureTarget::Texture2D, 0);
        }
    }

    unsafe fn do_fill(&self, call: &Call) {
        let paths = &self.paths[call.path_offset..call.path_offset + call.path_count];

        opengl_rs::enable(opengl_rs::def::EnableCap::StencilTest);
        opengl_rs::stencil_mask(0xff);
        opengl_rs::stencil_func(opengl_rs::def::All::Always, 0, 0xff);
        opengl_rs::color_mask(false, false, false, false);

        self.set_uniforms(call.uniform_offset, call.image);

        opengl_rs::stencil_op_separate(
            opengl_rs::def::All::Front,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::IncrWrap,
        );
        opengl_rs::stencil_op_separate(
            opengl_rs::def::All::Back,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::DecrWrap,
        );
        opengl_rs::disable(opengl_rs::def::EnableCap::CullFace);
        for path in paths {
            opengl_rs::draw_arrays(
                opengl_rs::def::BeginMode::TriangleFan,
                path.fill_offset as i32,
                path.fill_count as i32,
            );
        }
        opengl_rs::enable(opengl_rs::def::EnableCap::CullFace);

        opengl_rs::color_mask(true, true, true, true);

        self.set_uniforms(call.uniform_offset + 1, call.image);

        opengl_rs::stencil_func(opengl_rs::def::All::Equal, 0x00, 0xff);
        opengl_rs::stencil_op(
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
        );
        for path in paths {
            opengl_rs::draw_arrays(
                opengl_rs::def::BeginMode::TriangleStrip,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }

        opengl_rs::stencil_func(opengl_rs::def::All::Notequal, 0x00, 0xff);
        opengl_rs::stencil_op(
            opengl_rs::def::All::Zero,
            opengl_rs::def::All::Zero,
            opengl_rs::def::All::Zero,
        );
        opengl_rs::draw_arrays(
            opengl_rs::def::BeginMode::TriangleStrip,
            call.triangle_offset as i32,
            call.triangle_count as i32,
        );

        opengl_rs::disable(opengl_rs::def::EnableCap::StencilTest);
    }

    unsafe fn do_convex_fill(&self, call: &Call) {
        let paths = &self.paths[call.path_offset..call.path_offset + call.path_count];
        self.set_uniforms(call.uniform_offset, call.image);
        for path in paths {
            opengl_rs::draw_arrays(
                opengl_rs::def::BeginMode::TriangleFan,
                path.fill_offset as i32,
                path.fill_count as i32,
            );
            if path.stroke_count > 0 {
                opengl_rs::draw_arrays(
                    opengl_rs::def::BeginMode::TriangleStrip,
                    path.stroke_offset as i32,
                    path.stroke_count as i32,
                );
            }
        }
    }

    unsafe fn do_stroke(&self, call: &Call) {
        let paths = &self.paths[call.path_offset..call.path_offset + call.path_count];

        opengl_rs::disable(opengl_rs::def::EnableCap::StencilTest);
        opengl_rs::stencil_mask(0xff);
        opengl_rs::stencil_func(opengl_rs::def::All::Equal, 0x0, 0xff);
        opengl_rs::stencil_op(
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Incr,
        );

        self.set_uniforms(call.uniform_offset + 1, call.image);
        for path in paths {
            opengl_rs::draw_arrays(
                opengl_rs::def::BeginMode::TriangleStrip,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }

        self.set_uniforms(call.uniform_offset, call.image);
        opengl_rs::stencil_func(opengl_rs::def::All::Equal, 0x0, 0xff);
        opengl_rs::stencil_op(
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
            opengl_rs::def::All::Keep,
        );
        for path in paths {
            opengl_rs::draw_arrays(
                opengl_rs::def::BeginMode::TriangleStrip,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }

        opengl_rs::color_mask(false, false, false, false);
        opengl_rs::stencil_func(opengl_rs::def::All::Always, 0x0, 0xff);
        opengl_rs::stencil_op(
            opengl_rs::def::All::Zero,
            opengl_rs::def::All::Zero,
            opengl_rs::def::All::Zero,
        );
        for path in paths {
            opengl_rs::draw_arrays(
                opengl_rs::def::BeginMode::TriangleStrip,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }
        opengl_rs::color_mask(true, true, true, true);

        opengl_rs::disable(opengl_rs::def::EnableCap::StencilTest);
    }

    unsafe fn do_triangles(&self, call: &Call) {
        self.set_uniforms(call.uniform_offset, call.image);
        opengl_rs::draw_arrays(
            opengl_rs::def::BeginMode::Triangles,
            call.triangle_offset as i32,
            call.triangle_count as i32,
        );
    }

    fn convert_paint(
        &self,
        paint: &Paint,
        scissor: &Scissor,
        width: f32,
        fringe: f32,
        stroke_thr: f32,
    ) -> FragUniforms {
        let mut frag = FragUniforms {
            scissor_mat: Default::default(),
            paint_mat: Default::default(),
            inner_color: premul_color(paint.inner_color),
            outer_color: premul_color(paint.outer_color),
            scissor_ext: Default::default(),
            scissor_scale: Default::default(),
            extent: Default::default(),
            radius: 0.0,
            feather: 0.0,
            stroke_mult: 0.0,
            stroke_thr,
            tex_type: 0,
            type_: 0,
        };

        if scissor.extent.width < -0.5 || scissor.extent.height < -0.5 {
            frag.scissor_ext[0] = 1.0;
            frag.scissor_ext[1] = 1.0;
            frag.scissor_scale[0] = 1.0;
            frag.scissor_scale[1] = 1.0;
        } else {
            frag.scissor_mat = xform_to_3x4(scissor.xform.inverse());
            frag.scissor_ext[0] = scissor.extent.width;
            frag.scissor_ext[1] = scissor.extent.height;
            frag.scissor_scale[0] = (scissor.xform.0[0] * scissor.xform.0[0]
                + scissor.xform.0[2] * scissor.xform.0[2])
                .sqrt()
                / fringe;
            frag.scissor_scale[1] = (scissor.xform.0[1] * scissor.xform.0[1]
                + scissor.xform.0[3] * scissor.xform.0[3])
                .sqrt()
                / fringe;
        }

        frag.extent = [paint.extent.width, paint.extent.height];
        frag.stroke_mult = (width * 0.5 + fringe * 0.5) / fringe;

        let mut invxform = Transform::default();

        if let Some(img) = paint.image {
            if let Some(texture) = self.textures.get(img) {
                if texture.flags.contains(ImageFlags::FLIPY) {
                    let m1 = Transform::translate(0.0, frag.extent[1] * 0.5) * paint.xform;
                    let m2 = Transform::scale(1.0, -1.0) * m1;
                    let m1 = Transform::translate(0.0, -frag.extent[1] * 0.5) * m2;
                    invxform = m1.inverse();
                } else {
                    invxform = paint.xform.inverse();
                };

                frag.type_ = ShaderType::FillImage as i32;
                match texture.texture_type {
                    TextureType::RGBA => {
                        frag.tex_type = if texture.flags.contains(ImageFlags::PREMULTIPLIED) {
                            0
                        } else {
                            1
                        }
                    }
                    TextureType::Alpha => frag.tex_type = 2,
                }
            }
        } else {
            frag.type_ = ShaderType::FillGradient as i32;
            frag.radius = paint.radius;
            frag.feather = paint.feather;
            invxform = paint.xform.inverse();
        }

        frag.paint_mat = xform_to_3x4(invxform);

        frag
    }

}

impl renderer::Renderer for Renderer {
    fn edge_antialias(&self) -> bool {
        true
    }

    fn create_texture(
        &mut self,
        texture_type: TextureType,
        width: usize,
        height: usize,
        flags: ImageFlags,
        data: Option<&[u8]>,
    ) -> anyhow::Result<ImageId> {
        let tex = {
            let tex = opengl_rs::gen_texture();
            opengl_rs::bind_texture(opengl_rs::def::TextureTarget::Texture2D, tex);
            opengl_rs::pixel_storei(opengl_rs::def::All::UnpackAlignment, 1);

            match texture_type {
                TextureType::RGBA => {
                    opengl_rs::tex_image_2d(
                        opengl_rs::def::TextureTarget::Texture2D,
                        0,
                        opengl_rs::def::PixelInternalFormat::Rgba,
                        width as i32,
                        height as i32,
                        0,
                        opengl_rs::def::PixelFormat::Rgba,
                        opengl_rs::def::PixelType::UnsignedByte,
                        data,
                    );
                }
                TextureType::Alpha => {
                    opengl_rs::tex_image_2d(
                        opengl_rs::def::TextureTarget::Texture2D,
                        0,
                        opengl_rs::def::PixelInternalFormat::R8,
                        width as i32,
                        height as i32,
                        0,
                        opengl_rs::def::PixelFormat::Red,
                        opengl_rs::def::PixelType::UnsignedByte,
                        data,
                    );
                }
            }

            if flags.contains(ImageFlags::NEAREST) {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureMinFilter,
                    opengl_rs::def::TextureFilter::Nearest as _,
                );
            } else {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureMinFilter,
                    opengl_rs::def::TextureFilter::Linear as _,
                );
            }

            // if flags.contains(ImageFlags::GENERATE_MIPMAPS) {
            //     if flags.contains(ImageFlags::NEAREST) {
            //         opengl_rs::tex_parameter_i(
            //             opengl_rs::def::TextureTarget::Texture2D,
            //             opengl_rs::def::TextureParameterName::TextureMinFilter,
            //             opengl_rs::def::TextureFilter::NearestMipmapNearest as _,
            //         );
            //     } else {
            //         opengl_rs::tex_parameter_i(
            //             opengl_rs::def::TextureTarget::Texture2D,
            //             opengl_rs::def::TextureParameterName::TextureMinFilter,
            //             opengl_rs::def::TextureFilter::LinearMipmapLinear as _,
            //         );
            //     }
            // } else {
            //     if flags.contains(ImageFlags::NEAREST) {
            //         opengl_rs::tex_parameter_i(
            //             opengl_rs::def::TextureTarget::Texture2D,
            //             opengl_rs::def::TextureParameterName::TextureMinFilter,
            //             opengl_rs::def::TextureFilter::Nearest as _,
            //         );
            //     } else {
            //         opengl_rs::tex_parameter_i(
            //             opengl_rs::def::TextureTarget::Texture2D,
            //             opengl_rs::def::TextureParameterName::TextureMinFilter,
            //             opengl_rs::def::TextureFilter::Linear as _,
            //         );
            //     }
            // }

            if flags.contains(ImageFlags::NEAREST) {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureMagFilter,
                    opengl_rs::def::TextureFilter::Nearest as _,
                );
            } else {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureMagFilter,
                    opengl_rs::def::TextureFilter::Linear as _,
                );
            }

            if flags.contains(ImageFlags::REPEATX) {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureWrapS,
                    opengl_rs::def::TextureWrapMode::Repeat as _,
                );
            } else {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureWrapS,
                    opengl_rs::def::TextureWrapMode::ClampToEdge as _,
                );
            }

            if flags.contains(ImageFlags::REPEATY) {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureWrapT,
                    opengl_rs::def::TextureWrapMode::Repeat as _,
                );
            } else {
                opengl_rs::tex_parameter_i(
                    opengl_rs::def::TextureTarget::Texture2D,
                    opengl_rs::def::TextureParameterName::TextureWrapT,
                    opengl_rs::def::TextureWrapMode::ClampToEdge as _,
                );
            }

            opengl_rs::pixel_storei(opengl_rs::def::All::UnpackAlignment, 4);

            if flags.contains(ImageFlags::GENERATE_MIPMAPS) {
                opengl_rs::generate_mipmap(opengl_rs::def::GenerateMipmapTarget::Texture2D);
            }

            opengl_rs::bind_texture(opengl_rs::def::TextureTarget::Texture2D, 0);
            tex
        };

        let id = self.textures.insert(Texture {
            tex,
            width,
            height,
            texture_type,
            flags,
        });
        Ok(id)
    }

    fn delete_texture(&mut self, img: ImageId) -> anyhow::Result<()> {
        if let Some(texture) = self.textures.get(img) {
            opengl_rs::delete_texture(texture.tex);
            self.textures.remove(img);
            Ok(())
        } else {
            bail!("texture '{}' not found", img);
        }
    }

    fn update_texture(
        &mut self,
        img: ImageId,
        x: usize,
        y: usize, 
        width: usize,
        height: usize,
        data: &[u8],
    ) -> anyhow::Result<()> {
        if let Some(texture) = self.textures.get(img) {
            opengl_rs::bind_texture(opengl_rs::def::TextureTarget::Texture2D, texture.tex);
            opengl_rs::pixel_storei(opengl_rs::def::All::UnpackAlignment, 1);

            match texture.texture_type {
                TextureType::RGBA => opengl_rs::tex_sub_image_2d(
                    opengl_rs::def::TextureTarget::Texture2D,
                    0,
                    x as _,
                    y as _,
                    width as i32,
                    height as i32,
                    opengl_rs::def::PixelFormat::Rgba,
                    opengl_rs::def::PixelType::UnsignedByte,
                    Some(data),
                ),
                TextureType::Alpha => opengl_rs::tex_sub_image_2d(
                    opengl_rs::def::TextureTarget::Texture2D,
                    0,
                    x as i32,
                    y as i32,
                    width as i32,
                    height as i32,
                    opengl_rs::def::PixelFormat::Red,
                    opengl_rs::def::PixelType::UnsignedByte,
                    Some(data),
                ),
            }

            opengl_rs::pixel_storei(opengl_rs::def::All::UnpackAlignment, 4);
            opengl_rs::bind_texture(opengl_rs::def::TextureTarget::Texture2D, 0);
            Ok(())
        } else {
            bail!("texture '{}' not found", img);
        }
    }

    fn texture_size(&self, img: ImageId) -> anyhow::Result<(usize, usize)> {
        if let Some(texture) = self.textures.get(img) {
            Ok((texture.width, texture.height))
        } else {
            bail!("texture '{}' not found", img);
        }
    }

    fn viewport(&mut self, extent: Extent, _device_pixel_ratio: f32) -> anyhow::Result<()> {
        self.view = extent;
        Ok(())
    }

    fn cancel(&mut self) -> anyhow::Result<()> {
        self.vertexes.clear();
        self.paths.clear();
        self.calls.clear();
        self.uniforms.clear();
        Ok(())
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        todo!()
        // if !self.calls.is_empty() {
        //     unsafe {
        //         gl::UseProgram(self.shader.prog);

        //         gl::Enable(gl::CULL_FACE);
        //         gl::CullFace(gl::BACK);
        //         gl::FrontFace(gl::CCW);
        //         gl::Enable(gl::BLEND);
        //         gl::Disable(gl::DEPTH_TEST);
        //         gl::Disable(gl::SCISSOR_TEST);
        //         gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        //         gl::StencilMask(0xffffffff);
        //         gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
        //         gl::StencilFunc(gl::ALWAYS, 0, 0xffffffff);
        //         gl::ActiveTexture(gl::TEXTURE0);
        //         gl::BindTexture(gl::TEXTURE_2D, 0);

        //         gl::BindBuffer(gl::UNIFORM_BUFFER, self.frag_buf);
        //         gl::BufferData(
        //             gl::UNIFORM_BUFFER,
        //             self.uniforms.len() as isize,
        //             self.uniforms.as_ptr() as *const c_void,
        //             gl::STREAM_DRAW,
        //         );

        //         gl::BindVertexArray(self.vert_arr);
        //         gl::BindBuffer(gl::ARRAY_BUFFER, self.vert_buf);
        //         gl::BufferData(
        //             gl::ARRAY_BUFFER,
        //             (self.vertexes.len() * std::mem::size_of::<Vertex>()) as isize,
        //             self.vertexes.as_ptr() as *const c_void,
        //             gl::STREAM_DRAW,
        //         );
        //         gl::EnableVertexAttribArray(0);
        //         gl::EnableVertexAttribArray(1);
        //         gl::VertexAttribPointer(
        //             0,
        //             2,
        //             gl::FLOAT,
        //             gl::FALSE,
        //             std::mem::size_of::<Vertex>() as i32,
        //             std::ptr::null(),
        //         );
        //         gl::VertexAttribPointer(
        //             1,
        //             2,
        //             gl::FLOAT,
        //             gl::FALSE,
        //             std::mem::size_of::<Vertex>() as i32,
        //             (2 * std::mem::size_of::<f32>()) as *const c_void,
        //         );

        //         gl::Uniform1i(self.shader.loc_tex, 0);
        //         gl::Uniform2fv(
        //             self.shader.loc_viewsize,
        //             1,
        //             &self.view as *const Extent as *const f32,
        //         );

        //         gl::BindBuffer(gl::UNIFORM_BUFFER, self.frag_buf);

        //         for call in &self.calls {
        //             let blend = &call.blend_func;

        //             gl::BlendFuncSeparate(
        //                 blend.src_rgb,
        //                 blend.dst_rgb,
        //                 blend.src_alpha,
        //                 blend.dst_alpha,
        //             );

        //             match call.call_type {
        //                 CallType::Fill => self.do_fill(&call),
        //                 CallType::ConvexFill => self.do_convex_fill(&call),
        //                 CallType::Stroke => self.do_stroke(&call),
        //                 CallType::Triangles => self.do_triangles(&call),
        //             }
        //         }

        //         gl::DisableVertexAttribArray(0);
        //         gl::DisableVertexAttribArray(1);
        //         gl::BindVertexArray(0);
        //         gl::Disable(gl::CULL_FACE);
        //         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //         gl::UseProgram(0);
        //         gl::BindTexture(gl::TEXTURE_2D, 0);
        //     }
        // }

        // self.vertexes.clear();
        // self.paths.clear();
        // self.calls.clear();
        // self.uniforms.clear();
        // Ok(())
    }

    #[allow(unused_variables)]
    fn fill(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        bounds: Bounds,
        paths: &[Path],
    ) -> anyhow::Result<()> {
        let mut call = Call {
            call_type: CallType::Fill,
            image: paint.image,
            path_offset: self.paths.len(),
            path_count: paths.len(),
            triangle_offset: 0,
            triangle_count: 4,
            uniform_offset: 0,
            blend_func: composite_operation.into(),
        };

        if paths.len() == 1 && paths[0].convex {
            call.call_type = CallType::ConvexFill;
        }

        let mut offset = self.vertexes.len();
        for path in paths {
            let fill = path.get_fill();
            let mut gl_path = GLPath {
                fill_offset: 0,
                fill_count: 0,
                stroke_offset: 0,
                stroke_count: 0,
            };

            if !fill.is_empty() {
                gl_path.fill_offset = offset;
                gl_path.fill_count = fill.len();
                self.vertexes.extend(fill);
                offset += fill.len();
            }

            let stroke = path.get_stroke();
            if !stroke.is_empty() {
                gl_path.stroke_offset = offset;
                gl_path.stroke_count = stroke.len();
                self.vertexes.extend(stroke);
                offset += stroke.len();
            }

            self.paths.push(gl_path);
        }

        if call.call_type == CallType::Fill {
            call.triangle_offset = offset;
            self.vertexes
                .push(Vertex::new(bounds.max.x, bounds.max.y, 0.5, 1.0));
            self.vertexes
                .push(Vertex::new(bounds.max.x, bounds.min.y, 0.5, 1.0));
            self.vertexes
                .push(Vertex::new(bounds.min.x, bounds.max.y, 0.5, 1.0));
            self.vertexes
                .push(Vertex::new(bounds.min.x, bounds.min.y, 0.5, 1.0));

        } else {
        }

        self.calls.push(call);
        Ok(())
    }

    #[allow(unused_variables)]
    fn stroke(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        stroke_width: f32,
        paths: &[Path],
    ) -> anyhow::Result<()> {
        let call = Call {
            call_type: CallType::Stroke,
            image: paint.image,
            path_offset: self.paths.len(),
            path_count: paths.len(),
            triangle_offset: 0,
            triangle_count: 0,
            uniform_offset: 0,
            blend_func: composite_operation.into(),
        };

        let mut offset = self.vertexes.len();
        for path in paths {
            let mut gl_path = GLPath {
                fill_offset: 0,
                fill_count: 0,
                stroke_offset: 0,
                stroke_count: 0,
            };

            let stroke = path.get_stroke();
            if !stroke.is_empty() {
                gl_path.stroke_offset = offset;
                gl_path.stroke_count = stroke.len();
                self.vertexes.extend(stroke);
                offset += stroke.len();
                self.paths.push(gl_path);
            }
        }

        self.calls.push(call);
        Ok(())
    }

    fn triangles(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        vertexes: &[Vertex],
    ) -> anyhow::Result<()> {
        let call = Call {
            call_type: CallType::Triangles,
            image: paint.image,
            path_offset: 0,
            path_count: 0,
            triangle_offset: self.vertexes.len(),
            triangle_count: vertexes.len(),
            uniform_offset: self.uniforms.len(),
            blend_func: composite_operation.into(),
        };

        self.calls.push(call);
        self.vertexes.extend(vertexes);

        let mut uniforms = self.convert_paint(paint, scissor, 1.0, 1.0, -1.0);
        uniforms.type_ = ShaderType::Image as i32;
        Ok(())
    }
}

#[allow(dead_code)]
fn shader_error(shader: gl::types::GLuint, filename: &str) -> anyhow::Error {
    let err_msg = opengl_rs::get_shader_information(shader).unwrap();
    anyhow!("failed to compile shader: {}: {}", filename, err_msg)
}

#[allow(dead_code)]
fn program_error(prog: gl::types::GLuint) -> anyhow::Error {
    let err_msg = opengl_rs::get_program_information(prog).unwrap();
    anyhow!("failed to link program: {}", err_msg)
}

fn convert_blend_factor(factor: BlendFactor) -> gl::types::GLenum {
    match factor {
        BlendFactor::Zero => opengl_rs::def::BlendingFactor::Zero as _,
        BlendFactor::One => opengl_rs::def::BlendingFactor::One as _,
        BlendFactor::SrcColor => opengl_rs::def::BlendingFactor::SrcColor as _,
        BlendFactor::OneMinusSrcColor => opengl_rs::def::BlendingFactor::OneMinusSrcColor as _,
        BlendFactor::DstColor => opengl_rs::def::BlendingFactor::DstColor as _,
        BlendFactor::OneMinusDstColor => opengl_rs::def::BlendingFactor::OneMinusDstColor as _,
        BlendFactor::SrcAlpha => opengl_rs::def::BlendingFactor::SrcAlpha as _,
        BlendFactor::OneMinusSrcAlpha => opengl_rs::def::BlendingFactor::OneMinusSrcAlpha as _,
        BlendFactor::DstAlpha => opengl_rs::def::BlendingFactor::DstAlpha as _,
        BlendFactor::OneMinusDstAlpha => opengl_rs::def::BlendingFactor::OneMinusDstAlpha as _,
        BlendFactor::SrcAlphaSaturate => opengl_rs::def::BlendingFactor::SrcAlphaSaturate as _,
    }
}

#[inline]
fn premul_color(color: Color) -> Color {
    Color {
        r: color.r * color.a,
        g: color.g * color.a,
        b: color.b * color.a,
        a: color.a,
    }
}

#[inline]
fn xform_to_3x4(xform: Transform) -> [f32; 12] {
    let mut m = [0f32; 12];
    let t = &xform.0;
    m[0] = t[0];
    m[1] = t[1];
    m[2] = 0.0;
    m[3] = 0.0;
    m[4] = t[2];
    m[5] = t[3];
    m[6] = 0.0;
    m[7] = 0.0;
    m[8] = t[4];
    m[9] = t[5];
    m[10] = 1.0;
    m[11] = 0.0;
    m
}
