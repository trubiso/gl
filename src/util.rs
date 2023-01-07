use crate::shader::{Shader, uniform::Uniform};

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn clamp_mut<T: PartialOrd>(val: &mut T, min: T, max: T) -> &mut T {
    if *val < min {
        *val = min
    } else if *val > max {
        *val = max
    }
    val
}

pub trait MatrixUtils {
    fn to_slice(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const f32,
                std::mem::size_of_val(self) / std::mem::size_of::<f32>(),
            )
        }
    }

    fn from_val(val: f32) -> Self;

    fn identity() -> Self where Self: Sized {
        Self::from_val(1.0)
    }

    fn use_in_shader(self, ctx: &glow::Context, shader: &Shader, name: &str) where Self: Uniform, Self: Sized {
        shader.set_uniform(ctx, name, self);
    }
}

impl MatrixUtils for glm::Mat2 {
    fn from_val(val: f32) -> Self {
        glm::mat2(val, 0.0, 0.0, val)
    }
}
impl MatrixUtils for glm::Mat3 {
    fn from_val(val: f32) -> Self {
        glm::mat3(val, 0.0, 0.0, 0.0, val, 0.0, 0.0, 0.0, val)
    }
}
impl MatrixUtils for glm::Mat4 {
    fn from_val(val: f32) -> Self {
        glm::mat4(val, 0.0, 0.0, 0.0, 0.0, val, 0.0, 0.0, 0.0, 0.0, val, 0.0, 0.0, 0.0, 0.0, val)
    }
}

pub fn texture_num_to_u32(tnum: u32) -> u8 {
    (tnum - glow::TEXTURE0) as u8 // COULD POTENTIALLY BREAK
}

pub fn u32_to_texture_num(num: u8) -> u32 {
    num as u32 + glow::TEXTURE0 // COULD POTENTIALLY BREAK
}

pub trait Mat4Utils {
    fn translate(&mut self, coords: glm::Vec3) -> &mut Self;
    fn rotate(&mut self, angle: f32, axis: glm::Vec3) -> &mut Self;
}

impl Mat4Utils for glm::Mat4 {
    fn translate(&mut self, coords: glm::Vec3) -> &mut Self {
        *self = glm::ext::translate(self, coords);
        self
    }

    fn rotate(&mut self, angle: f32, axis: glm::Vec3) -> &mut Self {
        *self = glm::ext::rotate(self, angle, axis);
        self
    }
}
