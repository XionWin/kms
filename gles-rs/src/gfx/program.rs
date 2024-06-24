use libc::*;

use crate::{
    attach_shader, check_link_status, create_program, delete_program, link_program, use_program,
};

#[derive(Debug)]
pub struct GfxProgram {
    pub(crate) id: c_uint,
    pub(crate) vertex_shader: super::GfxShader,
    pub(crate) fragment_shader: super::GfxShader,
    pub(crate) textures: Vec<super::GfxTexture>,
}

impl GfxProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        Self {
            id: create_program(),
            vertex_shader: super::GfxShader::new(
                crate::def::ShaderType::VertexShader,
                vertex_shader_path,
            )
            .load(),
            fragment_shader: super::GfxShader::new(
                crate::def::ShaderType::FragmentShader,
                fragment_shader_path,
            )
            .load(),
            textures: Vec::new(),
        }
    }

    pub fn get_id(&self) -> c_uint {
        self.id
    }

    pub fn active(&self) {
        attach_shader(self.id, self.vertex_shader.id);
        attach_shader(self.id, self.fragment_shader.id);
        link_program(self.id);
        use_program(self.id);
        check_link_status(self.id);
    }

    pub fn add_texture(&mut self, texture: super::GfxTexture) {
        self.textures.push(texture)
    }
}

impl Drop for GfxProgram {
    fn drop(&mut self) {
        delete_program(self.id)
    }
}
