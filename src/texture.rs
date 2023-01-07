use glow::{NativeTexture, HasContext};

use crate::{shader::Shader, util::{u32_to_texture_num}};

#[derive(Debug)]
pub struct Texture {
  id: NativeTexture,
  num: u8,
  gl_num: u32
}

impl Texture {
    fn create_texture(ctx: &glow::Context, image: image::DynamicImage, num: u32) -> glow::NativeTexture {
        let texture = unsafe { ctx.create_texture() }.unwrap();

        unsafe {
            ctx.active_texture(num);
            ctx.bind_texture(glow::TEXTURE_2D, Some(texture));

            ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST_MIPMAP_NEAREST as i32);
            ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

            let width = image.width() as i32;
            let height = image.height() as i32;

            let a;

            let (image_format, image_type, data) = match &image {
                image::DynamicImage::ImageRgba8(data) => (glow::RGBA, glow::UNSIGNED_BYTE, &**data),
                image::DynamicImage::ImageRgb8(data) => (glow::RGB, glow::UNSIGNED_BYTE, &**data),
                _ => (glow::RGBA, glow::UNSIGNED_BYTE, {
                    a = image.into_rgba8();
                    &*a
                })
            };

            ctx.tex_image_2d(glow::TEXTURE_2D, 0, image_format as i32, width, height, 0, image_format, image_type, Some(data));
            ctx.generate_mipmap(glow::TEXTURE_2D)
        }

        texture
    }

    pub fn new(ctx: &glow::Context, image: image::DynamicImage, num: u8) -> Self {
        let gl_num = u32_to_texture_num(num);
        let texture = Self::create_texture(ctx, image, gl_num);
        Self {
            id: texture,
            num,
            gl_num
        }
    }

    pub fn from_file(ctx: &glow::Context, path: &str, num: u8) -> Self {
        let img = image::io::Reader::open(path).unwrap().decode().unwrap();
        let gl_num = u32_to_texture_num(num);
        let texture = Self::create_texture(ctx, img, gl_num);
        Self {
            id: texture,
            num,
            gl_num
        }
    }

    pub fn bind(&self, ctx: &glow::Context) {
        unsafe {
            ctx.active_texture(self.gl_num);
            ctx.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }

    pub fn use_in_shader(&self, ctx: &glow::Context, shader: &Shader) {
        let mut name_str = "tex".to_owned();
        name_str.push_str(&self.num.to_string());
        shader.set_uniform(ctx, &name_str, self.num as i32);
    }
}