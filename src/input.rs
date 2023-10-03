use crate::rendering::camera::CAMERA_SENSITIVITY;

/// A struct that manages the input of the game.
pub struct Input {
    /// The last position of the mouse.
    pub last_mouse: (f32, f32),
    /// Whether the mouse is captured.
    pub escaped: bool,
}

impl Input {
    /// Sets the last mouse position.
    pub fn set_last_mouse(&mut self, x: f32, y: f32) {
        self.last_mouse = (x, y);
    }

    /// Returns the last mouse position.
    pub fn last_mouse(&self) -> (f32, f32) {
        self.last_mouse
    }

    /// Handles mouse input.
    pub fn mouse_move(&mut self, x: f32, y: f32, callback: &mut dyn FnMut(f32, f32)) {
        if self.escaped {
            return;
        }

        let (last_x, last_y) = self.last_mouse;

        let x_offset = x - last_x;
        let y_offset = last_y - y;

        callback(x_offset * CAMERA_SENSITIVITY, y_offset * CAMERA_SENSITIVITY);

        self.last_mouse = (x, y);
    }

    /// Handles keyboard input. (Doesn't include movement)
    pub fn key(&mut self, key: glfw::Key, action: glfw::Action, window: &mut glfw::Window) {
        match (key, action) {
            (glfw::Key::Q, glfw::Action::Press) => window.set_should_close(true),
            (glfw::Key::Escape, glfw::Action::Press) => {
                self.toggle_capture(window);
            }
            _ => {}
        }
    }

    /// Toggle the mouse capture.
    pub fn toggle_capture(&mut self, window: &mut glfw::Window) {
        if !self.escaped {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        } else {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        self.escaped = !self.escaped;
    }
}
