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
