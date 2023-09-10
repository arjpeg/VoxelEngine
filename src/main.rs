mod buffer;

pub mod camera;
mod image;
mod shader;
mod shader_program;

use camera::Camera;
use rand::Rng;
use shader_program::ShaderProgram;
use std::mem::size_of;

#[allow(unused_imports)]
use nalgebra_glm as glm;

use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};

use crate::buffer::{vao::VAO, vbo::VBO, vertex::Vertex};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn key_is_down(window: &glfw::Window, key: Key) -> bool {
    window.get_key(key) == Action::Press
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "Hello OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Listen to events
    window.set_key_polling(true);

    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    // Enable depth testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Load the shaders
    let shader_program = ShaderProgram::load();

    // Generate random cube positions
    let cube_positions = {
        let mut cube_positions = Vec::new();

        for _ in 0..20 {
            let x: f32 = rng.gen_range(-5.0..5.0);
            let y: f32 = rng.gen_range(-5.0..5.0);
            let z: f32 = rng.gen_range(-5.0..5.0);

            // calculate rotation
            let x_rot: f32 = rng.gen_range(0.0..360.0);

            let rotation = glm::rotate(
                &glm::identity(),
                x_rot.to_radians(),
                &glm::vec3(1.0, 0.0, 0.0),
            );

            cube_positions.push((glm::vec3(x, y, z), rotation));
        }

        cube_positions
    };

    // Create the verticies
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
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 3.0), 45.0);
    let camera_sensitivity = 0.01f32;

    let projection_matrix = camera.get_projection_matrix(ASPECT_RATIO);

    let mut last_x = WIDTH as f32 / 2.0;
    let mut last_y = HEIGHT as f32 / 2.0;

    // Track delta time
    let mut delta_time;
    let mut last_frame = 0.0f32;

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();

        // Calculate delta time
        let time = glfw.get_time() as f32;

        delta_time = time - last_frame;
        last_frame = time;

        unsafe {
            shader_program.use_program();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind uniforms
            shader_program.set("view", camera.get_view_matrix().into());
            shader_program.set("projection", projection_matrix.into());

            // Draw the triangles
            vao.bind();

            for (position, rotation) in cube_positions.iter() {
                let model_matrix = glm::translate(&glm::identity(), &position);
                let model_matrix = model_matrix * rotation;

                shader_program.set("model", model_matrix.into());

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // check for errors
            let mut error: GLenum = gl::GetError();

            while error != gl::NO_ERROR {
                println!("Error: {}", error);
                error = gl::GetError();
            }
        }

        // Handle events
        {
            let camera_speed = 10.0 * delta_time;

            if key_is_down(&window, Key::W) {
                camera.move_in_dir(
                    glm::normalize(&glm::vec3(camera.front.x, 0.0, camera.front.z)) * camera_speed,
                )
            }

            if key_is_down(&window, Key::S) {
                camera.move_in_dir(
                    glm::normalize(&glm::vec3(camera.front.x, 0.0, camera.front.z))
                        * camera_speed
                        * -1.0,
                )
            }

            if key_is_down(&window, Key::A) {
                camera.move_in_dir(
                    glm::normalize(&glm::cross(&camera.front, &camera.up)) * camera_speed * -1.0,
                )
            }

            if key_is_down(&window, Key::D) {
                camera.move_in_dir(
                    glm::normalize(&glm::cross(&camera.front, &camera.up)) * camera_speed * 1.0,
                )
            }

            if window.get_key(Key::Space) == Action::Press {
                camera.move_in_dir(glm::vec3(0.0, 1.0, 0.0) * camera_speed);
            }

            if window.get_key(Key::LeftShift) == Action::Press {
                camera.move_in_dir(glm::vec3(0.0, -1.0, 0.0) * camera_speed);
            }

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Key(key, _, action, _) => match (key, action) {
                        (Key::Q, Action::Press) => window.set_should_close(true),
                        _ => {}
                    },
                    WindowEvent::CursorPos(x, y) => {
                        let x_offset = x as f32 - last_x;
                        let y_offset = last_y - y as f32;

                        last_x = x as f32;
                        last_y = y as f32;

                        camera.rotate(x_offset * camera_sensitivity, y_offset * camera_sensitivity);
                    }
                    _ => {}
                };
            }
        }
    }
}
