use glfw::Key;
use nalgebra_glm as glm;

use crate::utils::key_is_down;

pub const CAMERA_SENSITIVITY: f32 = 0.007;
pub const CAMERA_SPEED: f32 = 10.0;

pub struct Camera {
    /// The position of the camera
    pub position: glm::Vec3,
    /// The front vector of the camera
    pub front: glm::Vec3,

    /// The up vector of the camera
    pub up: glm::Vec3,

    /// The right vector of the camera
    pub right: glm::Vec3,

    /// The field of view of the camera
    pub fov: f32,
    /// The yaw of the camera
    pub yaw: f32,
    /// The pitch of the camera
    pub pitch: f32,
}

impl Camera {
    /// Creates a new camera
    pub fn new(position: glm::Vec3, fov: f32) -> Self {
        let up = glm::vec3(0.0, 1.0, 0.0);
        let right = glm::vec3(1.0, 0.0, 0.0);

        Self {
            position,
            front: glm::vec3(0.0, 0.0, -1.0),
            up,
            right,
            fov,
            yaw: -90.0f32.to_radians(),
            pitch: 0.0,
        }
    }

    /// Moves the camera in the given direction
    pub fn move_in_dir(&mut self, direction: glm::Vec3) {
        self.position += direction;
    }

    /// Rotates the camera by the given yaw and pitch
    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;

        self.pitch = self
            .pitch
            .clamp(-89.0f32.to_radians(), 89.0f32.to_radians());

        self.front = glm::vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
    }

    /// Gets the view matrix of the camera
    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    /// Gets the projection matrix of the camera
    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> glm::Mat4 {
        glm::perspective(aspect_ratio, self.fov.to_radians(), 0.1, 100.0)
    }

    /// Handles keyboard events
    pub fn handle_keyboard_input(&mut self, window: &glfw::Window, speed: f32) {
        let directions = &[
            (
                Key::A,
                glm::normalize(&glm::cross(&self.front, &self.up)) * -1.0,
            ),
            (Key::D, glm::normalize(&glm::cross(&self.front, &self.up))),
            (
                Key::W,
                glm::normalize(&glm::vec3(self.front.x, 0.0, self.front.z)),
            ),
            (
                Key::S,
                glm::normalize(&glm::vec3(self.front.x, 0.0, self.front.z)) * -1.0,
            ),
            (Key::Space, glm::vec3(0.0, 1.0, 0.0)),
            (Key::LeftShift, glm::vec3(0.0, -1.0, 0.0)),
        ];

        let speed_multiplier = if key_is_down(window, Key::LeftControl) {
            5.0
        } else {
            1.0
        };

        directions.map(|(key, direction)| {
            if key_is_down(window, key) {
                self.move_in_dir(direction * speed * speed_multiplier);
            }
        });
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(glm::vec3(0.0, 0.0, 0.0), 45.0)
    }
}
