mod buffers;
mod chunk;
mod input;
mod rendering;
mod timer;
mod utils;
mod voxel;

use std::sync::OnceLock;

use buffers::vbo::Vbo;
use chunk::Chunk;

use glfw::{Action, Context, Key, MouseButton, WindowEvent};
use nalgebra_glm as glm;

use owo_colors::OwoColorize;
use rendering::{camera::Camera, shader::shader_program::ShaderProgram};

use crate::{
    buffers::{ibo::Ibo, vao_builder::VaoBuilder},
    chunk::ChunkGenStrategy,
    input::InputManager,
    rendering::{camera::CAMERA_SPEED, mesh::MeshBuilder},
};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

pub static NOISE_SEED: OnceLock<u32> = OnceLock::new();
pub static NOISE: OnceLock<noise::Perlin> = OnceLock::new();

fn main() {
    // Initialize the noise seed
    NOISE_SEED.get_or_init(|| rand::random::<u32>());
    NOISE.get_or_init(|| noise::Perlin::new(*NOISE_SEED.get().unwrap()));

    println!(
        "Using noise speed: {}",
        NOISE_SEED.get().unwrap().cyan().bold()
    );

    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

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

    // Initalize the input capture
    let mut input = InputManager {
        last_mouse: (WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
        escaped: false,
    };

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Load the shaders
    let shader_program: ShaderProgram = Default::default();

    // Create new chunks
    let chunks = {
        let mut chunks = Vec::new();
        // let gen_strat = ChunkGenStrategy::FlatPlane(voxel::VoxelKind::Grass, 1);
        let gen_strat = ChunkGenStrategy::Perlin3d;

        for x in -10..10 {
            for z in -10..10 {
                let mut chunk = Chunk::new((x, z));
                gen_strat.apply(&mut chunk);
                chunks.push(chunk);
            }
        }

        chunks
    };

    // Create transformations
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 20.0), 45.0);
    let projection_matrix = camera.get_projection_matrix(ASPECT_RATIO);

    // Track delta time
    let mut delta_time;
    let mut last_frame = 0.0f32;

    let mut wire_frame = false;

    let mesh = MeshBuilder::new().build_mesh(&chunks);
    let mesh_vbo = Vbo::new(&mesh.vertices, gl::STATIC_DRAW);
    mesh_vbo.bind();
    get_gl_error!("Mesh VBO");

    let mesh_ibo = Ibo::new(&mesh.indices, gl::STATIC_DRAW);
    assert!(mesh.indices.len() % 6 == 0);
    mesh_ibo.bind();
    get_gl_error!("Mesh IBO");

    let vao = VaoBuilder::new().add_layer::<(f32, f32, f32)>(3).build();
    get_gl_error!("VAO");

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

        if 1.0 / delta_time < 30.0 {
            println!(
                "{}: FPS is very low ({})",
                "Warning".yellow().bold(),
                (1.0 / delta_time).green()
            );
        }

        unsafe {
            shader_program.use_program();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind uniforms
            shader_program.set_uniform("view", camera.get_view_matrix());
            shader_program.set_uniform("projection", projection_matrix);

            // Draw the triangles
            vao.bind();
            mesh_ibo.bind();

            gl::DrawElements(
                gl::TRIANGLES,
                mesh.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        // Handle input
        if !input.escaped {
            camera.handle_keyboard_input(&window, CAMERA_SPEED * delta_time);
        }

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    input.key(key, action, &mut window);

                    if let Key::F = key {
                        if let Action::Press = action {
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
                    }
                }
                WindowEvent::CursorPos(x, y) => {
                    input.mouse_move(x as f32, y as f32, &mut |x_offset, y_offset| {
                        camera.rotate(x_offset, y_offset);
                    });
                }
                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    if !input.escaped {
                        input.escaped = false;
                        window.set_cursor_mode(glfw::CursorMode::Disabled);
                    }
                }
                _ => {}
            };
        }
    }
}
