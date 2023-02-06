#include "Camera.h"
#include "Util.h"

void Camera::update_camera_vectors() {
	glm::vec3 new_front{
	    cos(this->yaw) * cos(this->pitch),
	    sin(this->pitch),
	    sin(this->yaw) * cos(this->pitch)};

	this->front = glm::normalize(new_front);
	this->right = glm::normalize(glm::cross(this->front, this->world_up));
	this->up    = glm::normalize(glm::cross(this->right, this->front));
}

void Camera::use_in_shader(Shader &shader) {
	shader.set_mat4("view", this->get_view_matrix());
	shader.set_mat4("projection", this->get_projection_matrix());
}

void Camera::move_direction(MovementDirection direction, double delta_time) {
	float velocity = this->speed * delta_time;
	// clang-format off
	switch (direction) {
		case Forward: 	this->position += velocity * this->front; 		break;
		case Backward: 	this->position -= velocity * this->front; 		break;
		case Left: 			this->position -= velocity * this->right; 		break;
		case Right: 		this->position += velocity * this->right; 		break;
		case WorldUp: 	this->position += velocity * this->world_up; 	break;
		case WorldDown: this->position -= velocity * this->world_up; 	break;
		case Up: 				this->position += velocity * this->up; 				break;
		case Down: 			this->position -= velocity * this->up; 				break;
	}
	// clang-format on
}

void Camera::rotate_with_offset(double x_offset, double y_offset, bool constrain_pitch) {
	this->yaw += x_offset * this->sensitivity;
	this->pitch += y_offset * this->sensitivity;

	if (constrain_pitch) clamp_ref(&this->pitch, 0.1 - glm::half_pi<double>(), glm::half_pi<double>() - 0.1);

	this->update_camera_vectors();
}

void Camera::scroll_fov(double y_offset) {
	this->fov = clamp(this->fov - y_offset, glm::pi<double>() / 180.0, glm::half_pi<double>());
}
