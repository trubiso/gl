#pragma once
#include <GL/glew.h>

class VertexArray {
private:
	unsigned ID;

public:
	inline VertexArray() { glGenVertexArrays(1, &this->ID); };

	inline void bind() { glBindVertexArray(this->ID); }
};
