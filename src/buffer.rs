use glow::{NativeVertexArray, HasContext, NativeBuffer};

use self::attrib::AttribNumber;

pub mod attrib;

struct VertexArrayAttrib {
    size: u32,
    data_type: u32,
    memsize: i32
}

impl VertexArrayAttrib {
    fn new(size: u32, data_type: u32, memsize: i32) -> Self {
        Self { size, data_type, memsize }
    }

    /// # Return value
    /// Returns `self.size` * `self.memsize`, useful for calculating offset of next attrib.
    fn apply(&self, ctx: &glow::Context, idx: u32, stride: u32, offset: u32) -> u32 {
        match self.data_type {
            glow::FLOAT => {
                if self.memsize == std::mem::size_of::<f32>() as i32 {
                    // F32 FN CALL
                    unsafe {
                        ctx.vertex_attrib_pointer_f32(idx, self.size as i32, self.data_type, false, stride as i32, offset as i32);
                    }
                } else {
                    // F64 FN CALL
                    unsafe {
                        ctx.vertex_attrib_pointer_f64(idx, self.size as i32, self.data_type, stride as i32, offset as i32);
                    }
                }
            },
            glow::INT => {
                // I32 FN CALL
                unsafe {
                    ctx.vertex_attrib_pointer_i32(idx, self.size as i32, self.data_type, stride as i32, offset as i32);
                }
            },
            _ => unreachable!()
        }

        // enable self
        unsafe {
            ctx.enable_vertex_attrib_array(idx);
        }

        self.size * self.memsize as u32
    }
}

pub struct VertexArrayObject {
    id: NativeVertexArray,
    attrib_vec: Vec<VertexArrayAttrib>
}

impl VertexArrayObject {
    pub fn new(ctx: &glow::Context) -> Self {
        let id = unsafe { ctx.create_vertex_array().unwrap() };
        Self {
            id,
            attrib_vec: Vec::new()
        }
    }

    pub fn bind(&self, ctx: &glow::Context) {
        unsafe {
            ctx.bind_vertex_array(Some(self.id));
        }
    }

    pub fn add_attrib<T: AttribNumber>(&mut self, size: u32) -> &mut Self {
        self.attrib_vec.push(VertexArrayAttrib::new(size, T::to_glow_type(), std::mem::size_of::<f32>() as i32));
        self
    }

    pub fn apply_attribs(&self, ctx: &glow::Context) {
        self.bind(ctx);
        let stride: u32 = self.attrib_vec.iter().map(|x| x.size * x.memsize as u32).sum();
        let mut offset = 0u32;
        for (idx, attrib) in self.attrib_vec.iter().enumerate() {
            offset += attrib.apply(ctx, idx as u32, stride, offset);
        }
    }
}

pub struct VertexBufferObject {
    id: NativeBuffer
}

impl VertexBufferObject {
    pub fn new(ctx: &glow::Context) -> Self {
        let id = unsafe { ctx.create_buffer().unwrap() };
        Self {
            id
        }
    }

    pub fn bind(&self, ctx: &glow::Context) {
        unsafe {
            ctx.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
        }
    }

    /// Note: binds self, may want to re-bind other VBO afterwards
    pub fn set_data<T: bytemuck::Pod>(&self, ctx: &glow::Context, val: &[T]) {
        self.bind(ctx);
        unsafe {
            ctx.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(val),
                glow::STATIC_DRAW,
            );
        }
    }
}