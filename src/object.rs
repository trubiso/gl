use glfw::Window;

use crate::{resource::ResourceManager, camera::Camera};

pub mod test_cube;

pub trait GameObject {
    /// Initializes VAO, VBO, EBO, shaders, textures... in resource manager
    /// to avoid recreating them afterwards in `new()`, especially
    /// shaders (compiling shaders is expensive!). This method does NOT
    /// create a new instance of the class.
    fn init(ctx: &glow::Context, rm: &mut ResourceManager);

    /// Draws the object. The GL context is provided for `Shader` &
    /// `VertexAttribArray` usage.
    fn draw(&self, ctx: &glow::Context, rm: &ResourceManager, cam: &dyn Camera, time: f32);

    /// Updates the object's state. This event may also be used to check
    /// whether keys are down or up (press and release events are handled
    /// in `handle_key_event()`). Delta time is sent for movement-related
    /// operations.
    fn update(&mut self, win: &Window, delta_time: f32, time: f32);

    /// Handles key event. This method is called upon key press, key
    /// release and key repeat (may be checked in `action`). Modifiers
    /// are also sent. Delta time is sent for movement-related
    /// operations.
    fn handle_key_event(&mut self, key: glfw::Key, action: glfw::Action, modifiers: glfw::Modifiers, delta_time: f32, time: f32);
}
