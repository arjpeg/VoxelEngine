use glfw::Context;

/// Manages the application state, including
/// the window, the renderer, and the event loop.
pub struct App {
    /// The window
    pub window: glfw::PWindow,
}

impl App {
    /// Creates a new window, initializes OpenGL, and returns the app.
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

        let (mut window, events) = glfw
            .create_window(800, 600, "Voxel", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);

        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Self { window }
    }

    /// Returns true if the window should close.
    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    /// Renders the frame.
    pub fn render(&mut self) {
        self.window.swap_buffers();
    }

    /// Updates all the components.
    pub fn update(&mut self) {}
}
