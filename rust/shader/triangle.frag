#version 330 core
out vec4 FragColor;

in vec2 tex_coord;

uniform sampler2D tex1;
uniform sampler2D tex2;
uniform float mix_val;
uniform float time;

void main() {
  vec4 tex = mix(texture(tex1, tex_coord), texture(tex2, tex_coord), mix_val);
  FragColor = tex;
}
