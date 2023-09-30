mod buffers;
pub mod chunk;
pub mod voxel;

use std::mem::size_of;

use buffers::{vao::Vao, vbo::Vbo, vertex::Vertex};
use chunk::Chunk;

use nalgebra_glm as glm;

mod rendering;

use rendering::{
    camera::Camera, shader::shader_program::ShaderProgram, shapes::cube::CUBE_POSITIONS,
};

use gl::types::*;
use glfw::{Action, Context, Key, MouseButton, WindowEvent};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn key_is_down(window: &glfw::Window, key: Key) -> bool {
    window.get_key(key) == Action::Press
}

fn get_error(label: &str) {
    let mut error: GLenum = unsafe { gl::GetError() };

    while error != gl::NO_ERROR {
        println!("Error ({}): {}", label, error);
        error = unsafe { gl::GetError() };
    }
}

fn main() {
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
    let shader_program: ShaderProgram = Default::default();

    // Create a new chunk
    let chunk = Chunk::new(glm::vec2(0.0, 0.0));

    // Create transformations
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 3.0), 45.0);
    let camera_sensitivity = 0.01f32;

    let projection_matrix = camera.get_projection_matrix(ASPECT_RATIO);

    let mut last_x = WIDTH as f32 / 2.0;
    let mut last_y = HEIGHT as f32 / 2.0;

    // Track delta time
    let mut delta_time;
    let mut last_frame = 0.0f32;

    let mut wire_frame = false;
    let mut last_wire_frame_timer = 0.0f32;

    let mut escaped = false;

    let cube_vbo = Vbo::new(&CUBE_POSITIONS, gl::STATIC_DRAW);

    get_error("Cube VBO");

    let mut cube_vao = Vao::new();

    cube_vbo.bind();
    cube_vao.bind();

    cube_vao.set_attribute(0, 3, gl::FLOAT, false, size_of::<Vertex>(), 0);

    get_error("Cube VAO");

    // Loop until the user closes the window
    while !window.should_close() {
        // Check if the user wants to toggle wireframe mode
        if key_is_down(&window, Key::F) && last_wire_frame_timer > 0.2 {
            last_wire_frame_timer = 0.0;

            if !wire_frame {
                unsafe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
            } else {
                unsafe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }

            wire_frame = !wire_frame;
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();

        // Calculate delta time
        let time = glfw.get_time() as f32;

        delta_time = time - last_frame;
        last_frame = time;

        println!("FPS: {}", 1.0 / delta_time);

        // Update the wireframe timer
        last_wire_frame_timer += delta_time;

        unsafe {
            shader_program.use_program();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind uniforms
            shader_program.set_uniform("view", camera.get_view_matrix());
            shader_program.set_uniform("projection", projection_matrix);

            // Bind the VAO
            cube_vao.bind();

            get_error("Uniforms");

            // Render the chunk
            for cube in chunk.cubes.iter() {
                shader_program.set_uniform(
                    "model",
                    glm::translate(
                        &glm::identity(),
                        &glm::vec3(cube.x as f32, cube.y as f32, cube.z as f32),
                    ),
                );

                gl::DrawArrays(gl::TRIANGLES, 0, 36);

                get_error("DrawArrays");
                // panic!();
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
                        (Key::Escape, Action::Press) => {
                            if !escaped {
                                window.set_cursor_mode(glfw::CursorMode::Normal);
                            } else {
                                window.set_cursor_mode(glfw::CursorMode::Disabled);
                            }

                            escaped = !escaped;
                        }
                        _ => {}
                    },
                    WindowEvent::CursorPos(x, y) => {
                        if escaped {
                            continue;
                        }

                        let x_offset = x as f32 - last_x;
                        let y_offset = last_y - y as f32;

                        last_x = x as f32;
                        last_y = y as f32;

                        camera.rotate(x_offset * camera_sensitivity, y_offset * camera_sensitivity);
                    }
                    WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                        if escaped {
                            escaped = false;
                            window.set_cursor_mode(glfw::CursorMode::Disabled);
                        }
                    }
                    _ => {}
                };
            }
        }
    }
}
