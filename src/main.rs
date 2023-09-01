mod buffer;

mod image;
mod shader;
mod shader_program;

use shader_program::ShaderProgram;
use std::{mem::size_of, ptr};

use nalgebra_glm as glm;

use gl::types::*;
use glfw::{ffi::glfwGetCursorPos, Action, Context, Key, MouseButton, WindowEvent};

use crate::{
    buffer::{ibo::IBO, vao::VAO, vbo::VBO, vertex::Vertex},
    image::Image,
};

fn load_shaders() -> ShaderProgram {
    let shader_program = ShaderProgram::new("./res/shaders/vertex.glsl", "./res/shaders/frag.glsl");

    shader_program
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(600, 600, "Hello OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Listen to events
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);

    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    // Load the shaders
    let shader_program = load_shaders();

    let verticies = VBO::new(
        &[
            Vertex::new([0.5, 0.5, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
            Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex::new([-0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0]),
        ],
        gl::STATIC_DRAW,
    );
    let indicies = IBO::new(
        &[
            1, 0, 2, // first triangle
            0, 3, 2, // second triangle
        ],
        gl::STATIC_DRAW,
    );
    let mut vao = VAO::new();

    verticies.bind();

    vao.set_attribute(0, 3, gl::FLOAT, false, 8 * size_of::<GLfloat>(), 0);

    vao.set_attribute(
        1,
        3,
        gl::FLOAT,
        false,
        8 * size_of::<GLfloat>(),
        3 * size_of::<GLfloat>(),
    );

    vao.set_attribute(
        2,
        2,
        gl::FLOAT,
        false,
        8 * size_of::<GLfloat>(),
        6 * size_of::<GLfloat>(),
    );

    let image = Image::new("./res/image/wall.jpg");
    let image2 = Image::new("./res/image/awesomeface.png");

    let mut scale_factor: f32 = 1.0;

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();

        // Draw the triangle
        unsafe {
            shader_program.use_program();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Set uniforms
            let time_value = glfw.get_time() as f32;

            let degrees = 50.0f32 * time_value;
            let radians = degrees.to_radians();

            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;

            glfwGetCursorPos(window.window_ptr(), &mut x, &mut y);

            // normalize
            let mouse_x = x / 600.0 * 2.0 - 1.0;
            let mouse_y = y / 600.0 * -2.0 + 1.0;

            let mouse_x = mouse_x as f32;
            let mouse_y = mouse_y as f32;

            let transform = glm::identity::<f32, 4>();
            let transform = glm::translate(&transform, &glm::vec3(mouse_x, mouse_y, 0.0));
            let transform = glm::rotate(&transform, radians, &glm::vec3(0.0, 0.0, 1.0));
            let transform = glm::scale(&transform, &glm::vec3(scale_factor, scale_factor, 1.0));

            shader_program.set_mat4("u_matrix", &transform);

            gl::ActiveTexture(gl::TEXTURE0);
            image.bind();

            gl::ActiveTexture(gl::TEXTURE1);
            image2.bind();

            // Draw the triangle
            vao.bind();
            indicies.bind();

            shader_program.set_int("u_texture", 0);
            shader_program.set_int("u_texture2", 1);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            // check for errors
            let mut error: GLenum = gl::GetError();

            while error != gl::NO_ERROR {
                println!("Error: {}", error);
                error = gl::GetError();
            }
        }

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);

            match event {
                WindowEvent::Key(Key::Q, _, Action::Press, _) => window.set_should_close(true),

                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    scale_factor += 0.1;
                }

                WindowEvent::MouseButton(MouseButton::Button2, Action::Press, _) => {
                    scale_factor -= 0.1;
                }

                _ => {}
            }
        }
    }
}
