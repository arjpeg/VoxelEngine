mod buffer;

mod image;
mod shader;
mod shader_program;

use rand::Rng;
use shader_program::ShaderProgram;
use std::mem::size_of;

#[allow(unused_imports)]
use nalgebra_glm as glm;

use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};

use crate::buffer::{vao::VAO, vbo::VBO, vertex::Vertex};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

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

    let cube_positions = {
        let mut cube_positions = Vec::new();

        for _ in 0..10 {
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
    let fovy = 45.0f32.to_radians();
    let near = 0.1;
    let far = 100.0;

    let projection_matrix = glm::perspective(ASPECT_RATIO, fovy, near, far);

    // Set up the camera
    let mut camera_position = glm::vec3(0.0, 0.0, 6.0);

    let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    let mut yaw = -90.0f32;
    let mut pitch = 0.0f32;

    let sensitivity = 0.1f32;
    let mut last_x = WIDTH as f32 / 2.0;
    let mut last_y = HEIGHT as f32 / 2.0;

    // Track delta time
    #[allow(unused_assignments)]
    let mut delta_time = 0.0f32;
    let mut last_frame = 0.0f32;

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();

        unsafe {
            shader_program.use_program();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let time = glfw.get_time() as f32;

            delta_time = time - last_frame;
            last_frame = time;

            // Calculate the view matrix
            camera_front = glm::vec3(
                yaw.to_radians().cos() * pitch.to_radians().cos(),
                pitch.to_radians().sin(),
                yaw.to_radians().sin() * pitch.to_radians().cos(),
            );

            camera_front.normalize_mut();

            let view_matrix = glm::look_at(
                &camera_position,
                &(camera_position + camera_front),
                &camera_up,
            );

            // Bind uniforms
            shader_program.set("view", view_matrix.into());
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
        let direction_map: [(&[Key], glm::TVec3<f32>); 4] = [
            (&[Key::A, Key::Left], glm::vec3(-0.1, 0.0, 0.0)),
            (&[Key::D, Key::Right], glm::vec3(0.1, 0.0, 0.0)),
            (&[Key::S, Key::Down], glm::vec3(0.0, 0.0, -0.1)),
            (&[Key::W, Key::Up], glm::vec3(0.0, 0.0, 0.1)),
        ];

        let camera_speed = 100.0 * delta_time;

        for (keys, direction) in direction_map.iter() {
            for key in keys.iter() {
                match window.get_key(*key) {
                    Action::Press | Action::Repeat => {
                        camera_position += {
                            match key {
                                Key::A | Key::D => {
                                    glm::normalize(&glm::cross(&camera_front, &camera_up))
                                        * camera_speed
                                        * direction.x
                                }
                                Key::W | Key::S => camera_front * camera_speed * direction.z,
                                _ => direction * camera_speed,
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(key, _, action, _) => match (key, action) {
                    (Key::Escape, Action::Press) => window.set_should_close(true),
                    _ => {}
                },
                WindowEvent::CursorPos(x, y) => {
                    let x_offset = x as f32 - last_x;
                    let y_offset = last_y - y as f32;

                    last_x = x as f32;
                    last_y = y as f32;

                    yaw += x_offset * sensitivity;
                    pitch += y_offset * sensitivity;
                }
                _ => {}
            };
        }
    }
}
