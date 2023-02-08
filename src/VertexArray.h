#pragma once
#include <GL/glew.h>
#include <vector>

typedef unsigned int uint; // this is unnecessary but vscode literally will NOT shut up

class VertexArray {
private:
	uint ID;

public:
	inline VertexArray() { glGenVertexArrays(1, &this->ID); };

	inline void bind() { glBindVertexArray(this->ID); }
};

class VertexArrayAttributeBuilder {
private:
	struct Attribute {
		uint size;
		GLenum gl_data_type;
		uint memory_size;
	};

	std::vector<Attribute> attributes;

	uint apply_attribute(uint index, uint stride, uint offset);
	void add_attribute(uint size, GLenum gl_data_type, uint memory_size);

public:
	VertexArrayAttributeBuilder() : attributes(){};

	inline VertexArrayAttributeBuilder &add_float_attribute(uint size) { this->add_attribute(size, GL_FLOAT, sizeof(float)); return *this; }

	inline VertexArrayAttributeBuilder &add_int_attribute(uint size) { this->add_attribute(size, GL_INT, sizeof(int)); return *this; }

	void apply_to(VertexArray &vertex_array);
};
