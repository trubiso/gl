#version 330 core
out vec4 FragColor;

in vec2 tex_coord;

uniform sampler2D tex1;
uniform sampler2D tex2;
uniform float mix_val;
uniform float time;

void main() {
  // vec4 tex = mix(texture(tex1, tex_coord), texture(tex2, tex_coord), mix_val);
  // float mv1 = (sin(time)        + 1.0f) / 2.0f;
  // float mv2 = (sin(time * 2.0f) + 1.0f) / 2.0f;
  // float mv3 = (sin(time * 3.0f) + 1.0f) / 2.0f;
  // float cx = mix(our_color.x, our_color.y + our_color.z, mv1 * mix_val);
  // float cy = mix(our_color.y, our_color.x + our_color.z, mv2 * mix_val);
  // float cz = mix(our_color.z, our_color.x + our_color.y, mv3 * mix_val);
  FragColor = vec4(mix_val, 1.0 - mix_val, tex_coord.x, 1.0);
}
