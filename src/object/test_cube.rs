use std::f32::consts::PI;

use glfw::{Window, Key};
use glow::HasContext;

use crate::{camera::Camera, resource::ResourceManager, util::{MatrixUtils, clamp_mut, Mat4Utils}, texture::Texture, shader::Shader, buffer::{VertexBufferObject, VertexArrayObject}};

pub struct TestCubeObject {
    mix_val: f32,
    rot_val: f32,
    pos: glm::Vec3
}

impl TestCubeObject {
    pub fn new(rot_val: f32, pos: glm::Vec3) -> Self {
        Self {
            mix_val: 0.0,
            rot_val,
            pos
        }
    }
}

impl super::GameObject for TestCubeObject {
    fn init(ctx: &glow::Context, rm: &mut ResourceManager) {
        #[rustfmt::skip]
        let vertices = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,
        
            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
        
            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,
        
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
        
            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
        
            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0f32
        ];

        let mut vao = VertexArrayObject::new(&ctx);
        vao.bind(&ctx);

        let vbo = VertexBufferObject::new(&ctx);
        vbo.set_data(&ctx, &vertices);

        vao.add_attrib::<f32>(3).add_attrib::<f32>(2).apply_attribs(&ctx);

        let texture0 = Texture::from_file(&ctx, "img/container.jpg", 0);
        let texture1 = Texture::from_file(&ctx, "img/awesome.png", 1);
        let shader = Shader::from_file(&ctx, "shader/triangle.vert", "shader/triangle.frag", None);

        shader.make_current(&ctx);
        texture0.use_in_shader(&ctx, &shader);
        texture1.use_in_shader(&ctx, &shader);

        rm.register_shader(shader, "test_cube_shader");
        rm.register_texture(texture0, "test_cube_texture0");
        rm.register_texture(texture1, "test_cube_texture1");
        rm.register_vao(vao, "test_cube_vao");
    }

    fn draw(&self, ctx: &glow::Context, rm: &ResourceManager, cam: &dyn Camera, time: f32) {
        let shader = rm.get_shader("test_cube_shader").unwrap();
        let texture0 = rm.get_texture("test_cube_texture0").unwrap();
        let texture1 = rm.get_texture("test_cube_texture1").unwrap();
        let vao = rm.get_vao("test_cube_vao").unwrap();

        texture0.bind(&ctx);
        texture1.bind(&ctx);

        vao.bind(&ctx);

        shader.make_current(&ctx);
        shader.set_uniform(&ctx, "mix_val", self.mix_val);

        cam.use_in_shader(ctx, shader);

        glm::Mat4::identity()
            .translate(self.pos)
            .rotate(PI * self.rot_val / 9.0, glm::vec3(1.0, 0.3, 0.5))
            .rotate(time, glm::vec3(0.5, 1.0, 0.0))
            .use_in_shader(&ctx, &shader, "model");

        unsafe {
            ctx.draw_arrays(glow::TRIANGLES, 0, 36);
        }
    }

    fn update(&mut self, win: &Window, delta_time: f32, _time: f32) {
        if win.get_key(Key::Up) == glfw::Action::Press { self.mix_val += 2.0 * delta_time; clamp_mut(&mut self.mix_val, 0.0, 1.0); }
        if win.get_key(Key::Down) == glfw::Action::Press { self.mix_val -= 2.0 * delta_time; clamp_mut(&mut self.mix_val, 0.0, 1.0); }
    }

    fn handle_key_event(&mut self, _key: glfw::Key, _action: glfw::Action, _modifiers: glfw::Modifiers, _delta_time: f32, _time: f32) {}
}
