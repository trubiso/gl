#pragma once
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include "Shader.h"

class Camera {
private:
	void update_camera_vectors();
	double screen_ratio;

public:
	enum MovementDirection {
		Forward,
		Backward,
		Left,
		Right,
		WorldUp,
		WorldDown,
		Up,
		Down
	};

	constexpr static double DEFAULT_YAW         = -glm::half_pi<double>();
	constexpr static double DEFAULT_PITCH       = 0.0;
	constexpr static double DEFAULT_SPEED       = 10.0;
	constexpr static double DEFAULT_SENSITIVITY = 1.0 / 400.0;
	constexpr static double DEFAULT_FOV         = glm::quarter_pi<double>();

	// clang-format pls don't align with the top ones
	double yaw         = DEFAULT_YAW;
	double pitch       = DEFAULT_PITCH;
	double speed       = DEFAULT_SPEED;
	double sensitivity = DEFAULT_SENSITIVITY;
	double fov         = DEFAULT_FOV;

	glm::vec3 position;
	glm::vec3 front;
	glm::vec3 up;
	glm::vec3 right;
	glm::vec3 world_up;

	Camera(glm::vec3 position = glm::vec3(0.0, 0.0, 0.0), glm::vec3 world_up = glm::vec3(0.0, 1.0, 0.0), double yaw = DEFAULT_YAW, double pitch = DEFAULT_PITCH)
	    : yaw(yaw), pitch(pitch), speed(DEFAULT_SPEED), sensitivity(DEFAULT_SENSITIVITY), fov(DEFAULT_FOV), position(position), front(0.0, 0.0, -1.0), up(0.0, 0.0, 0.0), right(0.0, 0.0, 0.0), world_up(world_up) {
		update_camera_vectors();
	}

	inline void set_screen_ratio(double new_screen_ratio) { this->screen_ratio = new_screen_ratio; }

	inline double get_screen_ratio() { return this->screen_ratio; }

	inline glm::mat4 get_view_matrix() { return glm::lookAt(this->position, this->position + this->front, this->up); }

	inline glm::mat4 get_projection_matrix() { return glm::perspective(this->fov, this->screen_ratio, 0.1, 100.0); }

	void use_in_shader(Shader &shader);

	void move_direction(MovementDirection direction, double delta_time);
	void rotate_with_offset(double x_offset, double y_offset, bool constrain_pitch = true);
	void scroll_fov(double y_offset);
};
