use std::fs;
use glow::{HasContext, NativeProgram, NativeUniformLocation};

pub mod uniform;

enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    id: NativeProgram,
}

impl Shader {
    fn create_shader(ctx: &glow::Context, code: &str, shader_type: ShaderType) -> glow::NativeShader {
        let shader = unsafe {
            ctx.create_shader(match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
            })
        }
        .unwrap();

        unsafe {
            ctx.shader_source(shader, code);
            ctx.compile_shader(shader);
            if !ctx.get_shader_compile_status(shader) {
                let log = ctx.get_shader_info_log(shader);
                panic!("{}", log);
            }
        }

        shader
    }

    pub fn new(ctx: &glow::Context, vert: &str, frag: &str) -> Self {
        let vert = Self::create_shader(ctx, vert, ShaderType::Vertex);
        let frag = Self::create_shader(ctx, frag, ShaderType::Fragment);

        let id = unsafe {
            let id = ctx.create_program().unwrap();
            ctx.attach_shader(id, vert);
            ctx.attach_shader(id, frag);
            ctx.link_program(id);
            if !ctx.get_program_link_status(id) {
                let log = ctx.get_program_info_log(id);
                panic!("{}", log);
            }
            ctx.delete_shader(vert);
            ctx.delete_shader(frag);
            id
        };

        Self {
            id
        }
    }

    pub fn from_file(ctx: &glow::Context, vert: &str, frag: &str) -> Self {
        Self::new(
            ctx,
            &fs::read_to_string(vert).unwrap(),
            &fs::read_to_string(frag).unwrap(),
        )
    }

    pub fn make_current(&self, ctx: &glow::Context) {
        unsafe { ctx.use_program(Some(self.id)); }
    }

    pub fn get_uniform_location(&self, ctx: &glow::Context, name: &str) -> NativeUniformLocation {
        unsafe { ctx.get_uniform_location(self.id, name) }.unwrap()
    }

    pub fn set_uniform<T: uniform::Uniform>(&self, ctx: &glow::Context, name: &str, value: T) {
        value.set(ctx, self.get_uniform_location(ctx, name));
    }
}
