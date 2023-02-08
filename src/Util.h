#pragma once
#include <GL/glew.h>
#include <string>

std::string read_file(const char *path);

template <typename T>
inline T clamp(T val, T min, T max) {
	if (val < min) return min;
	if (val > max) return max;
	return val;
}

template <typename T>
inline T clamp_ref(T *val, T min, T max) {
	if (*val < min) *val = min;
	if (*val > max) *val = max;
	return *val;
}

inline uint gl_texture_number_to_uint(GLenum texture_number) {
	return texture_number - GL_TEXTURE0;
}

inline GLenum uint_to_gl_texture_number(uint number) {
	return number + GL_TEXTURE0;
}
