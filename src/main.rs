use camera::{CameraMovementDirection, CameraEvent::KeyboardMovement};
use glfw::{Context, WindowEvent, Key, Action};
use glow::HasContext;
use util::clamp_mut;

use crate::{shader::Shader, camera::{Camera, FreeFlyCamera}};

pub mod camera;
pub mod shader;
pub mod util;
pub mod object;
pub mod resource;
pub mod texture;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_RATIO: f32 = (SCREEN_WIDTH as f32) / (SCREEN_HEIGHT as f32);

fn main() {
    // init
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // window
    let (mut window, receiver) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Rust OpenGL",
            glfw::WindowMode::Windowed,
        )
        .unwrap();
    window.make_current();

    // glow
    let ctx =
        unsafe { glow::Context::from_loader_function(|name: &str| window.get_proc_address(name)) };

    // :D
    unsafe {
        ctx.enable(glow::DEPTH_TEST);
    }
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // prepare viewport
    unsafe {
        ctx.viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    }

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    let shader = Shader::from_file(&ctx, "shader/triangle.vert", "shader/triangle.frag");

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

    let vao = unsafe { ctx.create_vertex_array().unwrap() };
    unsafe {
        ctx.bind_vertex_array(Some(vao));
    }

    let vbo = unsafe { ctx.create_buffer().unwrap() };
    unsafe {
        ctx.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        ctx.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            glow::STATIC_DRAW,
        );
    }

    // attributes
    unsafe {
        let float = std::mem::size_of::<f32>() as i32;

        ctx.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 5 * float, 0);
        ctx.enable_vertex_attrib_array(0);

        ctx.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 5 * float, 3 * float);
        ctx.enable_vertex_attrib_array(1);
    }

    // loading texture
    let texture0 = texture::Texture::from_file(&ctx, "img/container.jpg", 0);
    let texture1 = texture::Texture::from_file(&ctx, "img/awesome.png", 1);

    shader.make_current(&ctx);
    texture0.use_in_shader(&ctx, &shader);
    texture1.use_in_shader(&ctx, &shader);

    let mut delta_time;
    let mut last_frame = 0.0f64;

    let mut mix_val = 0.0f32;

    let mut wireframe_status: u32 = 0;

    let mut cam = FreeFlyCamera::default();

    let mut first_mouse = true;
    let mut last_x = SCREEN_WIDTH as f32 / 2.0;
    let mut last_y = SCREEN_HEIGHT as f32 / 2.0;

    let cube_positions = [
        glm::vec3( 0.0,  0.0,  0.0), 
        glm::vec3( 2.0,  5.0, -15.0), 
        glm::vec3(-1.5, -2.2, -2.5),  
        glm::vec3(-3.8, -2.0, -12.3),  
        glm::vec3( 2.4, -0.4, -3.5),  
        glm::vec3(-1.7,  3.0, -7.5),  
        glm::vec3( 1.3, -2.0, -2.5),  
        glm::vec3( 1.5,  2.0, -2.5), 
        glm::vec3( 1.5,  0.2, -1.5), 
        glm::vec3(-1.3,  1.0, -1.5)  
    ];

    while !window.should_close() {
        unsafe {
            ctx.clear_color(1.0, 0.776470588235, 0.0, 1.0);
            ctx.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            texture0.bind(&ctx);
            texture1.bind(&ctx);

            ctx.bind_vertex_array(Some(vao));
        }

        let time_value = glfw.get_time();
        delta_time = (time_value - last_frame) as f32;
        last_frame = time_value;

        shader.make_current(&ctx);
        shader.set_uniform(&ctx, "mix_val", mix_val);
        // shader.set_uniform(&ctx, "time", time_value as f32);

        let projection = glm::ext::perspective(cam.fov.to_radians(), SCREEN_RATIO, 0.1, 100.0);
        let view = cam.get_view_matrix();

        shader.set_uniform(&ctx, "projection", projection);
        shader.set_uniform(&ctx, "view", view);

        for i in 0..10 {
            let mut model = glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
            model = glm::ext::translate(&model, cube_positions[i]);
            let angle = 20.0 * i as f32;
            model = glm::ext::rotate(&model, angle.to_radians(), glm::vec3(1.0, 0.3, 0.5));
            model = glm::ext::rotate(&model, time_value as f32 * 50.0f32.to_radians(), glm::vec3(0.5, 1.0, 0.0));
            shader.set_uniform(&ctx, "model", model);

            unsafe {
                ctx.draw_arrays(glow::TRIANGLES, 0, 36);
            }
        }

        unsafe {
            ctx.bind_vertex_array(None);

            window.swap_buffers();
            glfw.poll_events();

            {
                let mut process = |key: Key, direction| {
                    if window.get_key(key) == glfw::Action::Press { cam.process_camera_event(KeyboardMovement(direction), delta_time); }
                };
                process(Key::W, CameraMovementDirection::Front);
                process(Key::S, CameraMovementDirection::Back);
                process(Key::A, CameraMovementDirection::Left);
                process(Key::D, CameraMovementDirection::Right);
                if window.get_key(Key::Up) == glfw::Action::Press { mix_val += 2.0 * delta_time; clamp_mut(&mut mix_val, 0.0, 1.0); }
                if window.get_key(Key::Down) == glfw::Action::Press { mix_val -= 2.0 * delta_time; clamp_mut(&mut mix_val, 0.0, 1.0); }
            }

            for (_, x) in glfw::flush_messages(&receiver) {
                match x {
                    WindowEvent::Close => window.set_should_close(true),
                    WindowEvent::Key(key, _, action, _modifiers) => {
                        if action != Action::Press {continue}
                        match key {
                            Key::Escape => window.set_should_close(true),
                            Key::Z => {
                                wireframe_status += 1;
                                wireframe_status = wireframe_status.rem_euclid(3);
                                match wireframe_status {
                                    0 => ctx.polygon_mode(glow::FRONT_AND_BACK, glow::FILL),
                                    1 => ctx.polygon_mode(glow::FRONT_AND_BACK, glow::LINE),
                                    2 => ctx.polygon_mode(glow::FRONT_AND_BACK, glow::POINT),
                                    _ => unreachable!()
                                }
                            }
                            _ => {}
                        }
                    },
                    WindowEvent::CursorPos(x, y) => {
                        if first_mouse {
                            last_x = x as f32;
                            last_y = x as f32;
                            first_mouse = false;
                        }

                        let x_off = x as f32 - last_x;
                        let y_off = last_y - y as f32;
                        last_x = x as f32;
                        last_y = y as f32;

                        cam.process_camera_event(camera::CameraEvent::MouseMovement(x_off, y_off), delta_time);
                    },
                    WindowEvent::Scroll(_x, y) => {
                        cam.process_camera_event(camera::CameraEvent::ScrollMovement(y as f32), delta_time);
                    },
                    _ => {}
                }
            }
        }
    }
}
