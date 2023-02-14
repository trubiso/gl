#pragma once
#include <GL/glew.h>
#include <glm/glm.hpp>

class Shader {
private:
	uint ID;
	static uint create_shader(const char *code, GLenum type);
	Shader(const char *vertex_shader_code, const char *fragment_shader_code, const char *geometry_shader_code, bool geometry_shader_exists);
	static Shader from_file(const char *vertex_shader_path, const char *fragment_shader_path, const char *geometry_shader_path, bool geometry_shader_exists);

public:
	Shader(const char *vertex_shader_code, const char *fragment_shader_code) : Shader(vertex_shader_code, fragment_shader_code, nullptr, false){};
	Shader(const char *vertex_shader_code, const char *fragment_shader_code, const char *geometry_shader_code) : Shader(vertex_shader_code, fragment_shader_code, geometry_shader_code, true){};

	inline static Shader from_file(const char *vertex_shader_path, const char *fragment_shader_path) { return Shader::from_file(vertex_shader_path, fragment_shader_path, nullptr, false); }

	inline static Shader from_file(const char *vertex_shader_path, const char *fragment_shader_path, const char *geometry_shader_path) { return Shader::from_file(vertex_shader_path, fragment_shader_path, geometry_shader_path, true); }

	inline void use() const { glUseProgram(this->ID); }

	inline uint get_uniform_location(const char *name) const { return glGetUniformLocation(this->ID, name); }

	inline void set(const char *name, bool value) { glUniform1i(this->get_uniform_location(name), value); }

	inline void set(const char *name, int value) { glUniform1i(this->get_uniform_location(name), value); }

	inline void set(const char *name, float value) { glUniform1f(this->get_uniform_location(name), value); }

	inline void set(const char *name, const glm::vec2 &value) { glUniform2fv(this->get_uniform_location(name), 1, &value[0]); }

	inline void set(const char *name, const glm::vec3 &value) { glUniform3fv(this->get_uniform_location(name), 1, &value[0]); }

	inline void set(const char *name, const glm::vec4 &value) { glUniform4fv(this->get_uniform_location(name), 1, &value[0]); }

	inline void set(const char *name, const glm::mat2 &mat) { glUniformMatrix2fv(this->get_uniform_location(name), 1, GL_FALSE, &mat[0][0]); }

	inline void set(const char *name, const glm::mat3 &mat) { glUniformMatrix3fv(this->get_uniform_location(name), 1, GL_FALSE, &mat[0][0]); }

	inline void set(const char *name, const glm::mat4 &mat) { glUniformMatrix4fv(this->get_uniform_location(name), 1, GL_FALSE, &mat[0][0]); }
};
