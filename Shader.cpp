#include "Util.h"
#include "Shader.h"
#include <fstream>
#include <map>
#include <sstream>
#include <iostream>

unsigned int Shader::create_shader(const char *code_path, int type) {
  int success;
  char log[512];
  std::string code_str = read_file(code_path);
  const char *code = code_str.c_str();
  unsigned int shader = glCreateShader(type);
  glShaderSource(shader, 1, &code, NULL);
  glCompileShader(shader);
  glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(shader, 512, NULL, log);
    std::cout << "[ERROR] Compilation of shader at " << code_path << " failed!\n" << log << std::endl;
  } else {
    std::cout << "[INFO] Compiled shader " << code_path << " with ID " << shader << std::endl;
  }
  return shader;
}

Shader::Shader(const char *vert_path, const char *frag_path) {
  int success;
  char log[512];
  unsigned int vert = create_shader(vert_path, GL_VERTEX_SHADER);
  unsigned int frag = create_shader(frag_path, GL_FRAGMENT_SHADER);

  ID = glCreateProgram();
  glAttachShader(ID, vert);
  glAttachShader(ID, frag);
  glLinkProgram(ID);
  glGetProgramiv(ID, GL_LINK_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(ID, 512, NULL, log);
    std::cout << "[ERROR] Couldn't link program!\n" << log << std::endl;
  } else {
    std::cout << "[INFO] Compiled program with ID " << ID << std::endl;
  }

  glDeleteShader(vert);
  glDeleteShader(frag);
}

void Shader::use() {
  glUseProgram(ID);
}

unsigned int Shader::get_uniform(const char *name) {
  return glGetUniformLocation(ID, name);
}

inline unsigned int Shader::get_uniform(std::string name) {
  return get_uniform(name.c_str());
}

void Shader::set_uniform(const char *name, UniformType type, void *value) {
  auto loc = get_uniform(name);
  switch (type) {
    case INT:   glUniform1i (loc, *((int *)          value)); break;
    case UINT:  glUniform1ui(loc, *((unsigned int *) value)); break;
    case FLOAT: glUniform1f (loc, *((float *)        value)); break;
    default: break;
  }
}

inline void Shader::set_uniform(std::string name, UniformType type, void *value) {
  return set_uniform(name.c_str(), type, value);
}

