use std::collections::HashMap;

use crate::{shader::Shader, texture::Texture, buffer::{VertexArrayObject}};

pub struct ResourceManager {
    shaders: HashMap<String, Shader>,
    textures: HashMap<String, Texture>,
    vaos: HashMap<String, VertexArrayObject>
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            textures: HashMap::new(),
            vaos: HashMap::new()
        }
    }

    pub fn register_shader(&mut self, shader: Shader, name: &str) {
        if self.has_shader(name) { panic!("Tried to re-register resource"); }
        self.shaders.insert(name.to_owned(), shader);
    }

    pub fn get_shader(&self, name: &str) -> Option<&Shader> {
        self.shaders.get(name)
    }

    pub fn has_shader(&self, name: &str) -> bool {
        self.shaders.contains_key(name)
    }

    pub fn register_texture(&mut self, texture: Texture, name: &str) {
        if self.has_texture(name) { panic!("Tried to re-register resource"); }
        self.textures.insert(name.to_owned(), texture);
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn has_texture(&self, name: &str) -> bool {
        self.textures.contains_key(name)
    }

    pub fn register_vao(&mut self, vao: VertexArrayObject, name: &str) {
        if self.has_vao(name) { panic!("Tried to re-register resource"); }
        self.vaos.insert(name.to_owned(), vao);
    }

    pub fn get_vao(&self, name: &str) -> Option<&VertexArrayObject> {
        self.vaos.get(name)
    }

    pub fn has_vao(&self, name: &str) -> bool {
        self.vaos.contains_key(name)
    }
}
