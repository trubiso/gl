use std::fs;
use glow::{HasContext, NativeProgram, NativeUniformLocation};

pub mod uniform;

enum ShaderType {
    Vertex,
    Fragment,
    Geometry
}

#[derive(Debug)]
pub struct Shader {
    id: NativeProgram
}

impl Shader {
    fn create_shader(ctx: &glow::Context, code: &str, shader_type: ShaderType) -> glow::NativeShader {
        let shader = unsafe {
            ctx.create_shader(match shader_type {
                ShaderType::Vertex => glow::VERTEX_SHADER,
                ShaderType::Fragment => glow::FRAGMENT_SHADER,
                ShaderType::Geometry => glow::GEOMETRY_SHADER
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

    pub fn new(ctx: &glow::Context, vert: &str, frag: &str, geom: Option<String>) -> Self {
        let geom_exists = geom.is_some();
        let vert = Self::create_shader(ctx, vert, ShaderType::Vertex);
        let frag = Self::create_shader(ctx, frag, ShaderType::Fragment);
        let geom = match geom {
            Some(path) => Some(Self::create_shader(ctx, &path, ShaderType::Geometry)),
            _ => None
        };

        let id = unsafe {
            let id = ctx.create_program().unwrap();
            ctx.attach_shader(id, vert);
            ctx.attach_shader(id, frag);
            if geom_exists { ctx.attach_shader(id, geom.unwrap()) };
            ctx.link_program(id);
            if !ctx.get_program_link_status(id) {
                let log = ctx.get_program_info_log(id);
                panic!("{}", log);
            }
            ctx.delete_shader(vert);
            ctx.delete_shader(frag);
            if geom_exists { ctx.delete_shader(geom.unwrap()) };
            id
        };

        Self {
            id
        }
    }

    pub fn from_file(ctx: &glow::Context, vert: &str, frag: &str, geom: Option<&str>) -> Self {
        Self::new(
            ctx,
            &fs::read_to_string(vert).unwrap(),
            &fs::read_to_string(frag).unwrap(),
            match geom {
                Some(path) => Some(fs::read_to_string(path).unwrap()),
                _ => None
            }
        )
    }

    pub fn make_current(&self, ctx: &glow::Context) {
        unsafe { ctx.use_program(Some(self.id)); }
    }

    pub fn get_uniform_location(&self, ctx: &glow::Context, name: &str) -> Option<NativeUniformLocation> {
        unsafe { ctx.get_uniform_location(self.id, name) }
    }

    pub fn set_uniform<T: uniform::Uniform>(&self, ctx: &glow::Context, name: &str, value: T) {
        if let Some(loc) = self.get_uniform_location(ctx, name) {
            value.set(ctx, loc);
        }
    }
}
