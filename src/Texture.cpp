#include "Texture.h"
#include <stb_image.h>
#include <iostream>
#include "Util.h"

Texture::Texture(unsigned char *image, GLenum texture_number, int width, int height, GLint internal_format) {
	unsigned int texture;
	glGenTextures(1, &texture);
	glActiveTexture(texture_number);
	glBindTexture(GL_TEXTURE_2D, texture);

	// TODO: customize these params
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST_MIPMAP_NEAREST);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

	glTexImage2D(GL_TEXTURE_2D, 0, internal_format, width, height, 0, internal_format, GL_UNSIGNED_BYTE, image);
	glGenerateMipmap(GL_TEXTURE_2D);

	this->ID        = texture;
	this->number    = gl_texture_number_to_uint(texture_number);
	this->gl_number = texture_number;
}

Texture Texture::from_file(const char *path, GLenum texture_number, GLint internal_format) {
	int width, height, nr_channels;
	unsigned char *data = stbi_load(path, &width, &height, &nr_channels, 0);
	if (!data) {
		std::cout << "[ERROR] Failed to load texture at \"" << path << "\" (" << stbi_failure_reason() << ')' << std::endl;
	}
	Texture texture = Texture(data, texture_number, width, height, internal_format);
	stbi_image_free(data);

	return texture;
}

void Texture::bind() {
	glActiveTexture(this->gl_number);
	glBindTexture(GL_TEXTURE_2D, this->ID);
}

void Texture::use_in_shader(Shader &shader) {
	std::string name = "tex";
	name += std::to_string(this->number + 1);
	shader.set_int(name.c_str(), this->number);
}
