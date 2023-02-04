#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <iostream>

unsigned SCREEN_WIDTH  = 800;
unsigned SCREEN_HEIGHT = 600;
double SCREEN_RATIO    = (double) SCREEN_WIDTH / (double) SCREEN_HEIGHT;

void key_callback(GLFWwindow *window, int key, int scancode, int action, int modifiers) {
	if (key == GLFW_KEY_ESCAPE) {
		glfwSetWindowShouldClose(window, true);
	}
}

void framebuffer_size_callback(GLFWwindow *window, int width, int height) {
	SCREEN_WIDTH  = width;
	SCREEN_HEIGHT = height;
	// TODO: resize camera viewport
	glViewport(0, 0, width, height);
}

void cursor_pos_callback(GLFWwindow *window, double xpos, double ypos) {
	// TODO: make
}

void scroll_callback(GLFWwindow *window, double xoffset, double yoffset) {
	// TODO: make
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

	// prepare viewport
	glViewport(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

	glfwSetKeyCallback(window, key_callback);
	glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);
	glfwSetCursorPosCallback(window, cursor_pos_callback);
	glfwSetScrollCallback(window, scroll_callback);

	// glfwSwapInterval() glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

	while (!glfwWindowShouldClose(window)) {
		glClearColor(1.0, 0.776470588235, 0.0, 1.0);
		glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

		glfwSwapBuffers(window);
		glfwPollEvents();
	}

	return 0;
}
