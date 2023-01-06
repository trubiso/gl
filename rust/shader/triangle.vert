#version 330 core
layout (location = 0) in vec3 Pos;
layout (location = 1) in vec2 TexCoord;

out vec2 tex_coord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
  vec4 p = model * vec4(Pos, 1.0);
  gl_Position = projection * view * p;
  tex_coord = TexCoord;
}
