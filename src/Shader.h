#pragma once
#include <GL/glew.h>

class Shader {
private:
	unsigned ID;
	static unsigned create_shader(const char *code, GLenum type);
	Shader(const char *vertex_shader_code, const char *fragment_shader_code, const char *geometry_shader_code, bool geometry_shader_exists);
	static Shader from_file(const char *vertex_shader_path, const char *fragment_shader_path, const char *geometry_shader_path, bool geometry_shader_exists);

public:
	Shader(const char *vertex_shader_code, const char *fragment_shader_code) : Shader(vertex_shader_code, fragment_shader_code, nullptr, false){};
	Shader(const char *vertex_shader_code, const char *fragment_shader_code, const char *geometry_shader_code) : Shader(vertex_shader_code, fragment_shader_code, geometry_shader_code, true){};

	inline static Shader from_file(const char *vertex_shader_path, const char *fragment_shader_path) { return Shader::from_file(vertex_shader_path, fragment_shader_path, nullptr, false); }

	inline static Shader from_file(const char *vertex_shader_path, const char *fragment_shader_path, const char *geometry_shader_path) { return Shader::from_file(vertex_shader_path, fragment_shader_path, geometry_shader_path, true); }

	inline void use() const { glUseProgram(this->ID); }

	inline unsigned get_uniform_location(const char *name) const { return glGetUniformLocation(this->ID, name); }
};
