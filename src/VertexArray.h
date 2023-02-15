#pragma once
#include <GL/glew.h>
#include <vector>

typedef unsigned int uint; // this is unnecessary but vscode literally will NOT shut up

class VertexArray {
private:
	unsigned ID;

public:
	inline VertexArray() { glGenVertexArrays(1, &this->ID); };

	inline void bind() { glBindVertexArray(this->ID); }
};

class VertexArrayAttributeBuilder {
private:
	struct Attribute {
		unsigned size;
		GLenum gl_data_type;
		unsigned memory_size;
	};

	std::vector<Attribute> attributes;

	unsigned apply_attribute(unsigned index, unsigned stride, unsigned offset);
	void add_attribute(unsigned size, GLenum gl_data_type, unsigned memory_size);

public:
	VertexArrayAttributeBuilder() : attributes(){};

	inline VertexArrayAttributeBuilder &add_float_attribute(unsigned size) { this->add_attribute(size, GL_FLOAT, sizeof(float)); return *this; }

	inline VertexArrayAttributeBuilder &add_int_attribute(unsigned size) { this->add_attribute(size, GL_INT, sizeof(int)); return *this; }

	void apply_to(VertexArray &vertex_array);
};
