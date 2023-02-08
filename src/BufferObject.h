#pragma once
#include <GL/glew.h>

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
	inline void set_data(T new_data[]) { glBufferData(this->get_gl_type(), sizeof(T), new_data, GL_STATIC_DRAW); /* TODO: customize GL_STATIC_DRAW perhaps */ }
};
