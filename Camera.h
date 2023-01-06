#pragma once
#include <glm/ext/matrix_transform.hpp>
#include <glm/glm.hpp>

enum CameraMovement {
  FORWARD,
  BACKWARD,
  LEFT,
  RIGHT
};

class Camera {
  public:
    constexpr static float YAW         = -90.0f;
    constexpr static float PITCH       =   0.0f;
    constexpr static float SPEED       =  10.0f;
    constexpr static float SENSITIVITY =   0.1f;
    constexpr static float FOV         =  45.0f;

    float movement_speed;
    float mouse_sensitivity;
    float fov;

    float yaw;
    float pitch;

    glm::vec3 position;
    glm::vec3 front;
    glm::vec3 up;
    glm::vec3 right;
    glm::vec3 world_up;

    Camera(glm::vec3 position = glm::vec3(0.0f, 0.0f, 0.0f), glm::vec3 up = glm::vec3(0.0f, 1.0f, 0.0f), float yaw = YAW, float pitch = PITCH);
    Camera(float posX, float posY, float posZ, float upX, float upY, float upZ, float yaw = YAW, float pitch = PITCH);

    inline glm::mat4 get_view_matrix() { return glm::lookAt(position, position + front, up); }

    void process_movement(CameraMovement direction, float delta_time);
    void process_mouse_movement(float x_off, float y_off, bool constrain_pitch = true);
    void process_mouse_scroll(float y_off);

  private:
    void update_camera_vectors();
};

