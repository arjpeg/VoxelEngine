mod buffer;

mod image;
mod shader;
mod shader_program;

use shader_program::ShaderProgram;
use std::mem::size_of;

#[allow(unused_imports)]
use nalgebra_glm as glm;

use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};

use crate::buffer::{vao::VAO, vbo::VBO, vertex::Vertex};

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

    // Enable depth testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Load the shaders
    let shader_program = ShaderProgram::load();

    let verticies = VBO::new(
        &[
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0, 1.0]),
            Vertex::new([0.5, -0.5, -0.5], [1.0, 0.0, 0.0]),
            Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 0.0]),
            Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 0.0]),
            Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0, 1.0]),
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 0.0]),
            Vertex::new([-0.5, 0.5, 0.5], [0.0, 1.0, 1.0]),
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0, 0.0]),
            Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 0.0]),
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0, 1.0]),
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0, 1.0]),
            Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0, 1.0]),
            Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 0.0]),
            Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0, 1.0]),
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0, 1.0]),
            Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
            Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0, 1.0]),
            Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0, 1.0]),
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0, 1.0]),
            Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]),
            Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0, 0.0]),
        ],
        gl::STATIC_DRAW,
    );
    let mut vao = VAO::new();

    verticies.bind();

    vao.set_attribute(0, 3, gl::FLOAT, false, 6 * size_of::<GLfloat>(), 0);

    vao.set_attribute(
        1,
        3,
        gl::FLOAT,
        false,
        6 * size_of::<GLfloat>(),
        3 * size_of::<GLfloat>(),
    );

    // Create transformations
    let model_matrix = glm::rotate(&glm::identity(), 0.0, &glm::vec3(1.0, 0.0, 0.0));

    let view_matrix = glm::rotate(
        &glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, -3.0)),
        15.0f32.to_radians(),
        &glm::vec3(1.0, 0.0, 0.0),
    );

    let aspect_ratio = 800 as f32 / 600 as f32;
    let fovy = 45.0f32.to_radians();
    let near = 0.1;
    let far = 100.0;

    let projection_matrix = glm::perspective(aspect_ratio, fovy, near, far);

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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let time = glfw.get_time() as f32;

            let view_matrix = glm::rotate(
                &view_matrix,
                (time * 50.0).to_radians(),
                &glm::vec3(0.0, 1.0, 0.0),
            );

            // Bind uniforms
            shader_program.set_mat4("model", &model_matrix);
            shader_program.set_mat4("view", &view_matrix);
            shader_program.set_mat4("projection", &projection_matrix);

            // Draw the triangle
            vao.bind();

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

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
                _ => {}
            }
        }
    }
}
