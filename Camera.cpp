#include "Camera.h"
#include "Util.h"
#include <glm/gtc/matrix_transform.hpp>
#include <math.h>

Camera::Camera(glm::vec3 position, glm::vec3 up, float yaw, float pitch) : movement_speed(SPEED), mouse_sensitivity(SENSITIVITY), fov(FOV), yaw(yaw), pitch(pitch), position(position), front(0.0f, 0.0f, -1.0f), world_up(up) {
  update_camera_vectors();
}
Camera::Camera(float posX, float posY, float posZ, float upX, float upY, float upZ, float yaw, float pitch) : movement_speed(SPEED), mouse_sensitivity(SENSITIVITY), fov(FOV), yaw(yaw), pitch(pitch), position(posX, posY, posZ), front(0.0f, 0.0f, -1.0f), world_up(upX, upY, upZ) {
  update_camera_vectors();
}

void Camera::process_movement(CameraMovement direction, float delta_time) {
  float velocity = movement_speed * delta_time;
  switch (direction) {
    case FORWARD : position += front * velocity; break;
    case BACKWARD: position -= front * velocity; break;
    case LEFT    : position -= right * velocity; break;
    case RIGHT   : position += right * velocity; break;
  }
}

void Camera::process_mouse_movement(float x_off, float y_off, bool constrain_pitch) {
  x_off *= mouse_sensitivity;
  y_off *= mouse_sensitivity;
  yaw   += x_off;
  pitch += y_off;

  if (constrain_pitch) {
    clamp_ref(&pitch, -89.0f, 89.0f);
  }

  update_camera_vectors();
}

void Camera::process_mouse_scroll(float y_off) {
  fov -= y_off;
  clamp_ref(&fov, 1.0f, 90.0f);
}

void Camera::update_camera_vectors() {
  glm::vec3 new_front;
  new_front.x = cos(glm::radians(yaw)) * cos(glm::radians(pitch));
  new_front.y = sin(glm::radians(pitch));
  new_front.z = sin(glm::radians(yaw)) * cos(glm::radians(pitch));

  front = glm::normalize(new_front);
  right = glm::normalize(glm::cross(front, world_up));
  up    = glm::normalize(glm::cross(right, front));
}

