#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <iostream>
#include "BufferObject.h"
#include "Camera.h"
#include "Debug.h"
#include "Shader.h"
#include "Texture.h"
#include "VertexArray.h"

unsigned SCREEN_WIDTH   = 800;
unsigned SCREEN_HEIGHT  = 600;
double SCREEN_RATIO = (double) SCREEN_WIDTH / (double) SCREEN_HEIGHT;

Camera cam{};

void check_down_keys(GLFWwindow *window, double delta_time) {
	auto check_key = [&](int key, Camera::MovementDirection direction) {
		if (glfwGetKey(window, key) == GLFW_PRESS) cam.move_direction(direction, delta_time);
	};

	check_key(GLFW_KEY_W, Camera::MovementDirection::Forward);
	check_key(GLFW_KEY_S, Camera::MovementDirection::Backward);
	check_key(GLFW_KEY_A, Camera::MovementDirection::Left);
	check_key(GLFW_KEY_D, Camera::MovementDirection::Right);
	check_key(GLFW_KEY_LEFT_SHIFT, Camera::MovementDirection::WorldDown);
	check_key(GLFW_KEY_SPACE, Camera::MovementDirection::WorldUp);
}

void key_callback(GLFWwindow *window, int key, int scancode, int action, int modifiers) {
	switch (key) {
	case GLFW_KEY_ESCAPE:
		glfwSetWindowShouldClose(window, true);
		break;
	}
}

void framebuffer_size_callback(GLFWwindow *window, int width, int height) {
	SCREEN_WIDTH  = width;
	SCREEN_HEIGHT = height;
	SCREEN_RATIO  = (double) width / (double) height;
	cam.set_screen_ratio(SCREEN_RATIO);
	glViewport(0, 0, width, height);
}

bool is_first_cursor_pos_callback = true;
double last_x = 0.0, last_y = 0.0;

void cursor_pos_callback(GLFWwindow *window, double xpos, double ypos) {
	if (is_first_cursor_pos_callback) {
		last_x = xpos;
		last_y = ypos;
		//
		is_first_cursor_pos_callback = false;
	}

	double x_offset = xpos - last_x;
	double y_offset = last_y - ypos;
	//
	last_x = xpos;
	last_y = ypos;

	cam.rotate_with_offset(x_offset, y_offset);
}

void scroll_callback(GLFWwindow *window, double xoffset, double yoffset) {
	cam.scroll_fov(yoffset);
}

void error_callback(int error, const char *message) {
	std::cerr << "[" << std::to_string(error) << "] " << message << std::endl;
}

int main() {
	// init
	if (!glfwInit()) {
		std::cout << "Failed to initialize GLFW";
		return -1;
	}
	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
	glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);

	// window
	GLFWwindow *window = glfwCreateWindow(800, 600, "OpenGL", NULL, NULL);
	if (window == NULL) {
		std::cout << "Failed to create GLFW window" << std::endl;
		glfwTerminate();
		return -1;
	}
	glfwMakeContextCurrent(window);

	// glew
	if (glewInit() != GLEW_OK) {
		std::cout << "Failed to initialize GLEW";
		return -1;
	}

	// :D
	glEnable(GL_DEPTH_TEST);
	// window.set_cursor_mode(glfw::CursorMode::Disabled);

	glEnable(GL_DEBUG_OUTPUT);
	glDebugMessageCallback(debug_message_callback, nullptr);
	glfwSetErrorCallback(error_callback);

	// prepare viewport
	glViewport(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

	glfwSetKeyCallback(window, key_callback);
	glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);
	glfwSetCursorPosCallback(window, cursor_pos_callback);
	glfwSetScrollCallback(window, scroll_callback);

	// glfwSwapInterval() glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

	Shader shader = Shader::from_file("shader/triangle1.vert", "shader/triangle1.frag");

	// textures
	Texture texture0 = Texture::from_file("img/container.jpg", GL_TEXTURE0);
	Texture texture1 = Texture::from_file("img/awesome.png", GL_TEXTURE1, GL_RGBA);

	shader.use();
	texture0.use_in_shader(shader);
	texture1.use_in_shader(shader);

	// vertex
	// clang-format off
	const float vertices[] = {
		-0.5f, -0.5f, -0.5f,  0.0f, 0.0f,
		 0.5f, -0.5f, -0.5f,  1.0f, 0.0f,
		 0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
		 0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
		-0.5f,  0.5f, -0.5f,  0.0f, 1.0f,
		-0.5f, -0.5f, -0.5f,  0.0f, 0.0f,

		-0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
		 0.5f, -0.5f,  0.5f,  1.0f, 0.0f,
		 0.5f,  0.5f,  0.5f,  1.0f, 1.0f,
		 0.5f,  0.5f,  0.5f,  1.0f, 1.0f,
		-0.5f,  0.5f,  0.5f,  0.0f, 1.0f,
		-0.5f, -0.5f,  0.5f,  0.0f, 0.0f,

		-0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
		-0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
		-0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
		-0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
		-0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
		-0.5f,  0.5f,  0.5f,  1.0f, 0.0f,

		 0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
		 0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
		 0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
		 0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
		 0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
		 0.5f,  0.5f,  0.5f,  1.0f, 0.0f,

		-0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
		 0.5f, -0.5f, -0.5f,  1.0f, 1.0f,
		 0.5f, -0.5f,  0.5f,  1.0f, 0.0f,
		 0.5f, -0.5f,  0.5f,  1.0f, 0.0f,
		-0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
		-0.5f, -0.5f, -0.5f,  0.0f, 1.0f,

		-0.5f,  0.5f, -0.5f,  0.0f, 1.0f,
		 0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
		 0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
		 0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
		-0.5f,  0.5f,  0.5f,  0.0f, 0.0f,
		-0.5f,  0.5f, -0.5f,  0.0f, 1.0f
	};
	// clang-format on

	const unsigned indices[] = {0, 1, 2, 1, 2, 3};

	VertexArray va{};
	va.bind();

	BufferObject vbo{BufferObjectType::Array};
	vbo.bind();
	vbo.set_data(vertices);

	BufferObject ebo{BufferObjectType::Element};
	ebo.bind();
	ebo.set_data(indices);

	VertexArrayAttributeBuilder()
	    .add_float_attribute(3)  // position (x, y, z)
	    .add_float_attribute(2)  // texture coordinates (x, y)
	    .apply_to(va);

	// clang-format off
	glm::vec3 cube_positions[] = {
		glm::vec3( 0.0f,  0.0f,  0.0f ), 
		glm::vec3( 2.0f,  5.0f, -15.0f), 
		glm::vec3(-1.5f, -2.2f, -2.5f ),  
		glm::vec3(-3.8f, -2.0f, -12.3f),  
		glm::vec3( 2.4f, -0.4f, -3.5f ),  
		glm::vec3(-1.7f,  3.0f, -7.5f ),  
		glm::vec3( 1.3f, -2.0f, -2.5f ),  
		glm::vec3( 1.5f,  2.0f, -2.5f ), 
		glm::vec3( 1.5f,  0.2f, -1.5f ), 
		glm::vec3(-1.3f,  1.0f, -1.5f )
	};
	// clang-format on

	double time_value = 0, last_frame = 0, delta_time = 0;

	while (!glfwWindowShouldClose(window)) {
		glClearColor(1.0, 0.776470588235, 0.0, 1.0);
		glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

		texture0.bind();
		texture1.bind();
		va.bind();

		time_value = glfwGetTime();
		delta_time = time_value - last_frame;
		last_frame = time_value;

		shader.use();
		shader.set("mix_val", 0.0f);
		shader.set("time", (float) time_value);

		cam.use_in_shader(shader);

		for (unsigned i = 0; i < 10; i++) {
			glm::mat4 model = glm::mat4(1.0f);
			// model           = glm::translate(model, cube_positions[i]);
			// model           = glm::rotate(model, glm::radians(20.0f * i), glm::vec3(1.0f, 0.3f, 0.5f));
			// model           = glm::rotate(model, (float) time_value * glm::radians(50.0f), glm::vec3(0.5f, 1.0f, 0.0f));
			// shader.set("model", model);

			glDrawArrays(GL_TRIANGLES, 0, 36);
		}
		glBindVertexArray(0);

		glfwSwapBuffers(window);
		glfwPollEvents();
		check_down_keys(window, delta_time);
	}

	return 0;
}
