pub trait AttribNumber {
    fn to_glow_type() -> u32;
}

impl AttribNumber for f32 {
    fn to_glow_type() -> u32 {
        glow::FLOAT
    }    
}

impl AttribNumber for f64 {
    fn to_glow_type() -> u32 {
        glow::FLOAT
    }    
}

impl AttribNumber for i32 {
    fn to_glow_type() -> u32 {
        glow::INT
    }    
}