#pragma once
#include <GL/glew.h>
#include "Shader.h"

// TODO: (huge todo) spritesheets, texture atlas, etc (maybe could be done inside of the shaders themselves, perhaps through uniforms, but i'd still like some kind of abstraction for the concept of a spritesheet. for now since i've got a small amount of textures i'm not starting yet)
// maybe i could even leave this class as a small abstraction for loading textures and binding them but then provide a SpritesheetTexture alongside it... we'll see later
class Texture {
private:
	unsigned ID;
	unsigned number;
	unsigned gl_number;

public:
	Texture(unsigned char *image, GLenum texture_number, int width, int height, GLint internal_format = GL_RGB);
	static Texture from_file(const char *path, GLenum texture_number, GLint internal_format = GL_RGB);

	void bind();
	void use_in_shader(Shader &shader);
};
