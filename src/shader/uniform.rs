use glow::{HasContext, NativeUniformLocation};

use crate::util::MatrixUtils;

pub trait Uniform {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation);
}

impl Uniform for f32 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_1_f32(Some(&loc), *self);
        }
    }
}

impl Uniform for i32 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_1_i32(Some(&loc), *self);
        }
    }
}

impl Uniform for u32 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_1_u32(Some(&loc), *self);
        }
    }
}

impl Uniform for glm::Vec2 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_2_f32(Some(&loc), self.x, self.y);
        }
    }
}

impl Uniform for glm::Vec3 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_3_f32(Some(&loc), self.x, self.y, self.z);
        }
    }
}

impl Uniform for glm::Vec4 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_4_f32(Some(&loc), self.x, self.y, self.z, self.w);
        }
    }
}

impl Uniform for glm::Mat2 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_matrix_2_f32_slice(Some(&loc), false, self.to_slice());
        }
    }
}

impl Uniform for glm::Mat3 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_matrix_3_f32_slice(Some(&loc), false, self.to_slice());
        }
    }
}

impl Uniform for glm::Mat4 {
    fn set(&self, ctx: &glow::Context, loc: NativeUniformLocation) {
        unsafe {
            ctx.uniform_matrix_4_f32_slice(Some(&loc), false, self.to_slice());
        }
    }
}
