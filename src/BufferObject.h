#pragma once
#include <GL/glew.h>

// TODO: assign to actual gl types instead of having to add GL_ARRAY_BUFFER
enum BufferObjectType {
	Array,
	Element
};

class BufferObject {
private:
	unsigned ID;
	BufferObjectType type;

public:
	inline BufferObject(BufferObjectType type) : type(type) { glGenBuffers(1, &this->ID); }

	inline GLenum get_gl_type() const { return this->type + GL_ARRAY_BUFFER; }
	inline BufferObjectType get_type() const { return this->type; }

	inline void bind() const { glBindBuffer(this->get_gl_type(), this->ID); }

	template <typename T>
	inline void set_data(T new_data[], size_t memory_size) { glBufferData(this->get_gl_type(), memory_size, new_data, GL_STATIC_DRAW); /* TODO: customize GL_STATIC_DRAW perhaps */ }
};
