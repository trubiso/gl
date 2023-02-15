cmake_minimum_required(VERSION 3.10)

project(GL)

set(CMAKE_CXX_FLAGS "-Wno-int-to-pointer-cast")

set(OpenGL_GL_PREFERENCE GLVND)
find_package(glfw3 3.3 REQUIRED)
find_package(GLEW REQUIRED)
find_package(OpenGL REQUIRED)
find_package(glm REQUIRED)

include_directories(include)
add_subdirectory(src)

