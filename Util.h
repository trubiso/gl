#pragma once
#include <string>

std::string read_file(const char *path);

template<typename T> inline T clamp(T val, T min, T max) {
  if (val < min) return min;
  if (val > max) return max;
  return val;
}

template<typename T> inline T clamp_ref(T *val, T min, T max) {
  if (*val < min) *val = min;
  if (*val > max) *val = max;
  return *val;
}

