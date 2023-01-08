use glm::{self, vec3, Vec3};

use crate::{util::{clamp_mut}, SCREEN_RATIO};

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 10.0;
const SENSITIVITY: f32 = 0.1;
const FOV: f32 = 45.0;

pub enum CameraEvent {
    KeyboardMovement(CameraMovementDirection),
    MouseMovement(f32, f32),
    ScrollMovement(f32),
}

pub enum CameraMovementDirection {
    Front,
    Back,
    Left,
    Right,
    Up,
    Down
}

pub trait Camera {
    fn get_view_matrix(&self) -> glm::Mat4;
    fn get_projection_matrix(&self) -> glm::Mat4;
    fn use_in_shader(&self, ctx: &glow::Context, shader: &crate::shader::Shader);
    fn process_camera_event(&mut self, event: CameraEvent, delta_time: f32);
}

pub struct FreeFlyCamera {
    pub speed: f32,
    pub sensitivity: f32,
    pub fov: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub pos: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub wup: Vec3,
}

impl Camera for FreeFlyCamera {
    fn get_view_matrix(&self) -> glm::Mat4 {
        glm::ext::look_at(self.pos, self.pos + self.front, self.up)
    }

    fn get_projection_matrix(&self) -> glm::Mat4 {
        glm::ext::perspective(self.fov.to_radians(), SCREEN_RATIO, 0.1, 100.0)
    }

    fn use_in_shader(&self, ctx: &glow::Context, shader: &crate::shader::Shader) {
        shader.set_uniform(ctx, "view", self.get_view_matrix());
        shader.set_uniform(ctx, "projection", self.get_projection_matrix());
    }

    fn process_camera_event(&mut self, event: CameraEvent, delta_time: f32) {
        match event {
            CameraEvent::KeyboardMovement(direction) => {
                let velocity = self.speed * delta_time;
                match direction {
                    CameraMovementDirection::Front => self.pos = self.pos + self.front * velocity,
                    CameraMovementDirection::Back => self.pos = self.pos - self.front * velocity,
                    CameraMovementDirection::Right => self.pos = self.pos + self.right * velocity,
                    CameraMovementDirection::Left => self.pos = self.pos - self.right * velocity,
                    CameraMovementDirection::Up => self.pos = self.pos + self.wup * velocity,
                    CameraMovementDirection::Down => self.pos = self.pos - self.wup * velocity,
                }
            }
            CameraEvent::MouseMovement(x, y) => {
                self.yaw += x * self.sensitivity;
                self.pitch += y * self.sensitivity;
                clamp_mut(&mut self.pitch, -89.9, 89.9);
                self.update_camera_vectors();
            }
            CameraEvent::ScrollMovement(y) => {
                self.fov -= y;
                clamp_mut(&mut self.fov, 1.0, 90.0);
            }
        };
    }
}

impl Default for FreeFlyCamera {
    fn default() -> Self {
        return Self::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
    }
}

impl FreeFlyCamera {
    fn update_camera_vectors(&mut self) {
        let mut new_front = vec3(0.0, 0.0, 0.0);
        new_front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        new_front.y = self.pitch.to_radians().sin();
        new_front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = glm::normalize(new_front);
        self.right = glm::normalize(glm::cross(self.front, self.wup));
        self.up = glm::normalize(glm::cross(self.right, self.front));
    }

    pub fn new(pos: Vec3, wup: Vec3) -> Self {
        let mut cam = Self {
            speed: SPEED,
            sensitivity: SENSITIVITY,
            fov: FOV,
            yaw: YAW,
            pitch: PITCH,
            pos,
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 0.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),
            wup,
        };
        cam.update_camera_vectors();
        cam
    }
}
