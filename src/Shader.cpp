#include "Shader.h"
#include <GL/glew.h>
#include <iostream>
#include "Util.h"

uint Shader::create_shader(const char *code, GLenum type) {
	uint shader = glCreateShader(type);
	glShaderSource(shader, 1, &code, NULL);
	glCompileShader(shader);

	int success;
	char log[512];
	glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
	if (!success) {
		glGetShaderInfoLog(shader, 512, NULL, log);
		std::cout << "[ERROR] Couldn't compile shader!\n"
		          << log << std::endl;
	} else {
		std::cout << "[INFO] Compiled shader with ID " << shader << std::endl;
	}

	return shader;
}

Shader::Shader(const char *vertex_shader_code, const char *fragment_shader_code, const char *geometry_shader_code, bool geometry_shader_exists) {
	const uint vertex_id   = create_shader(vertex_shader_code, GL_VERTEX_SHADER);
	const uint fragment_id = create_shader(fragment_shader_code, GL_FRAGMENT_SHADER);
	const uint geometry_id = geometry_shader_exists ? create_shader(geometry_shader_code, GL_GEOMETRY_SHADER) : 0;

	this->ID                   = glCreateProgram();

	glAttachShader(this->ID, vertex_id);
	glAttachShader(this->ID, fragment_id);
	if (geometry_shader_exists) glAttachShader(this->ID, geometry_id);

	glLinkProgram(this->ID);

	int success;
	char log[512];
	glGetProgramiv(this->ID, GL_LINK_STATUS, &success);
	if (!success) {
		glGetShaderInfoLog(ID, 512, NULL, log);
		std::cout << "[ERROR] Couldn't link program!\n"
		          << log << std::endl;
	} else {
		std::cout << "[INFO] Compiled program with ID " << ID << std::endl;
	}

	glDeleteShader(vertex_id);
	glDeleteShader(fragment_id);
	if (geometry_shader_exists) glDeleteShader(geometry_id);
}

Shader Shader::from_file(const char *vertex_shader_path, const char *fragment_shader_path, const char *geometry_shader_path, bool geometry_shader_exists) {
	auto vertex_str = read_file(vertex_shader_path);
	auto fragment_str = read_file(fragment_shader_path);
	if (geometry_shader_exists) {
		auto geometry_str = read_file(geometry_shader_path);
		return Shader(vertex_str.c_str(), fragment_str.c_str(), geometry_str.c_str(), true);
	}
	return Shader(vertex_str.c_str(), fragment_str.c_str(), nullptr, false);
}
