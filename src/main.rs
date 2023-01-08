use camera::{CameraMovementDirection, CameraEvent::KeyboardMovement};
use glfw::{Context, WindowEvent, Key, Action};
use glow::HasContext;
use object::{test_cube::{TestCubeObject}, GameObject};
use resource::ResourceManager;

use crate::{camera::{Camera, FreeFlyCamera}};

pub mod camera;
pub mod shader;
pub mod util;
pub mod object;
pub mod resource;
pub mod texture;
pub mod buffer;

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

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let mut rm = ResourceManager::new();

    let mut delta_time;
    let mut last_frame = 0.0f64;

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

    TestCubeObject::init(&ctx, &mut rm);
    let mut test_cubes: Vec<TestCubeObject> = cube_positions.iter().enumerate().map(|(idx, pos)| TestCubeObject::new(idx as f32, *pos)).collect();

    let mut frames = 0u32;
    let mut time_sf = 0f32;

    while !window.should_close() {
        unsafe {
            ctx.clear_color(1.0, 0.776470588235, 0.0, 1.0);
            ctx.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        let time_value = glfw.get_time();
        delta_time = (time_value - last_frame) as f32;
        last_frame = time_value;

        frames += 1;
        time_sf += delta_time;

        if time_sf > 1.0 {
            let fps = frames as f32 * (1.0 / time_sf);
            println!("{}FPS", fps);
            frames = 0;
            time_sf -= 1.0;
        }

        test_cubes.iter().for_each(|x| x.draw(&ctx, &rm, &cam, time_value as f32));

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
                process(Key::LeftShift, CameraMovementDirection::Down);
                process(Key::Space, CameraMovementDirection::Up);
                test_cubes.iter_mut().for_each(|x| x.update(&window, delta_time, time_value as f32));
            }

            for (_, x) in glfw::flush_messages(&receiver) {
                match x {
                    WindowEvent::Close => window.set_should_close(true),
                    WindowEvent::Key(key, _, action, _modifiers) => {
                        if action != Action::Press {continue}
                        match key {
                            Key::Escape => window.set_should_close(true),
                            Key::Tab => {
                                window.set_cursor_mode(match window.get_cursor_mode() {
                                    glfw::CursorMode::Normal => glfw::CursorMode::Disabled,
                                    _ => glfw::CursorMode::Normal
                                });
                            },
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
