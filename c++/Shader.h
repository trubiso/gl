#pragma once

#include <glad/gl.h>
#include <glm/glm.hpp>
#include <string>
#include <map>

class Shader {
  private:
    unsigned int create_shader(const char *code_path, int type);

  public:
    unsigned int ID;

    Shader(const char *vert_path, const char *frag_path);

    inline void use() const { glUseProgram(ID); }

    inline unsigned int get_uniform(const char *name) const { return glGetUniformLocation(ID, name); }

    inline void set_bool (const char *name, bool  value) const { glUniform1i(get_uniform(name), value); }
    inline void set_int  (const char *name, int   value) const { glUniform1i(get_uniform(name), value); }
    inline void set_float(const char *name, float value) const { glUniform1f(get_uniform(name), value); }

    inline void set_vec2 (const char *name, const glm::vec2 &value) const { glUniform2fv(get_uniform(name), 1, &value[0]); }
    inline void set_vec3 (const char *name, const glm::vec3 &value) const { glUniform3fv(get_uniform(name), 1, &value[0]); }
    inline void set_vec4 (const char *name, const glm::vec4 &value) const { glUniform4fv(get_uniform(name), 1, &value[0]); }

    inline void set_vec2 (const char *name, float x, float y)                   const { glUniform2f(get_uniform(name), x, y);       }
    inline void set_vec3 (const char *name, float x, float y, float z)          const { glUniform3f(get_uniform(name), x, y, z);    }
    inline void set_vec4 (const char *name, float x, float y, float z, float w) const { glUniform4f(get_uniform(name), x, y, z, w); }

    inline void set_mat2 (const char *name, const glm::mat2 &mat) const { glUniformMatrix2fv(get_uniform(name), 1, GL_FALSE, &mat[0][0]); }
    inline void set_mat3 (const char *name, const glm::mat3 &mat) const { glUniformMatrix3fv(get_uniform(name), 1, GL_FALSE, &mat[0][0]); }
    inline void set_mat4 (const char *name, const glm::mat4 &mat) const { glUniformMatrix4fv(get_uniform(name), 1, GL_FALSE, &mat[0][0]); }
};

