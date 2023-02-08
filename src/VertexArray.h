#pragma once
#include <GL/glew.h>

class VertexArray {
private:
	uint ID;

public:
	inline VertexArray() { glGenVertexArrays(1, &this->ID); };

	inline void bind() { glBindVertexArray(this->ID); }
};
