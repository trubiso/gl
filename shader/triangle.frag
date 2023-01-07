#version 330 core
out vec4 FragColor;

in vec2 tex_coord;

uniform sampler2D tex0;
uniform sampler2D tex1;
uniform float mix_val;
uniform float time;

void main() {
  vec4 tex = mix(texture(tex0, tex_coord), texture(tex1, tex_coord), mix_val);
  FragColor = tex;
}
