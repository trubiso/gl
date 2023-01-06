#version 330 core
out vec4 FragColor;

uniform float time;

void main() {
  float sin_time = sin(time)        + 0.5f;
  float sin2time = sin(time * 2.0f) + 0.5f;
  float sin3time = sin(time * 3.0f) + 0.5f;
  FragColor = vec4(sin_time, sin2time, sin3time, 1.0f);
}
