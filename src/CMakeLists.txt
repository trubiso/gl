add_library(stb_image stb_image.cpp)
add_executable(out main.cpp Shader.cpp Texture.cpp Camera.cpp Util.cpp VertexArray.cpp)
target_link_libraries(out stb_image glfw GL GLEW::GLEW)
