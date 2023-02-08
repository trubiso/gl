#include "VertexArray.h"
#include <numeric>

uint VertexArrayAttributeBuilder::apply_attribute(uint index, uint stride, uint offset) {
	auto &attribute = this->attributes[index];
	switch (attribute.gl_data_type) {
		case GL_FLOAT:
			glVertexAttribPointer(index, attribute.size, GL_FLOAT, false, stride, &offset);
			break;
		case GL_INT:
			glVertexAttribIPointer(index, attribute.size, GL_INT, stride, &offset);
			break;
		// TODO: add support for glVertexAttribLPointer(index, attribute.size, GL_DOUBLE, stride, &offset); (long pointer, double presumably)
	}

	return attribute.size * attribute.memory_size;
}

void VertexArrayAttributeBuilder::add_attribute(uint size, GLenum gl_data_type, uint memory_size) {
	this->attributes.push_back({
		size,
		gl_data_type,
		memory_size
	});
}

void VertexArrayAttributeBuilder::apply_to(VertexArray &vertex_array) {
	vertex_array.bind();
	const uint stride = std::accumulate(this->attributes.cbegin(), this->attributes.cend(), 0, [](auto a, auto b){ return a + b.memory_size * b.size; });
	uint offset = 0u;
	for (uint index = 0 ; index < this->attributes.size() ; index ++) {
		offset += this->apply_attribute(index, stride, offset);
	}
}
