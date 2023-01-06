#pragma once

#include <glad/gl.h>
#include <string>
#include <map>

enum UniformType {
  INT,
  UINT,
  FLOAT,
  IVEC2,
  IVEC3,
  IVEC4,
  UVEC2,
  UVEC3,
  UVEC4,
  VEC2,
  VEC3,
  VEC4
};

class Shader {
  private:
    unsigned int create_shader(const char *code_path, int type);

  public:
    unsigned int ID;

    Shader(const char *vert_path, const char *frag_path);

    void use();

    unsigned int get_uniform(const char *name);
    inline unsigned int get_uniform(std::string name);

    void set_uniform(const char *name, UniformType type, void *value);
    inline void set_uniform(std::string name, UniformType type, void *value);
};

