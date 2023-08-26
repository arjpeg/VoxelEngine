mod shader;

use owo_colors::OwoColorize;
use std::{mem, process::exit, ptr};

use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};
use shader::{Shader, ShaderKind};

fn load_shaders() -> GLuint {
    let shader_program: GLuint;

    unsafe {
        shader_program = gl::CreateProgram();

        let mut vertex_shader = Shader::new("./shaders/vertex.glsl", ShaderKind::Vertex);
        let mut fragment_shader = Shader::new("./shaders/frag.glsl", ShaderKind::Fragment);

        vertex_shader.compile();
        fragment_shader.compile();

        vertex_shader.attach(shader_program);
        fragment_shader.attach(shader_program);

        gl::LinkProgram(shader_program);

        // Check for errors
        let mut success = 1;
        let mut info_log: [GLchar; 512] = [0; 512];

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != 1 {
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr());

            let info_log: &[u8] =
                std::slice::from_raw_parts(info_log.as_ptr() as *const u8, info_log.len());

            println!("{} while linking shader program:", "Error".red().bold());
            println!("{}", std::str::from_utf8(&info_log).unwrap());

            exit(1)
        }
    }

    println!("Successfully loaded shaders...");

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

    let verticies: [GLfloat; 12] = [
        0.5, 0.5, 0.0, // Top right
        0.5, -0.5, 0.0, // Bottom right
        -0.5, -0.5, 0.0, // Bottom left
        -0.5, 0.5, 0.0, // Top left
    ];
    let indicies = [
        0, 1, 3, // First triangle
        1, 2, 3, // Second triangle
    ];

    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;

    unsafe {
        // Generates one buffer and stores its id in vbo
        gl::GenBuffers(1, &mut vbo);

        // Binds the buffer to the GL_ARRAY_BUFFER target
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Bind the verticies to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            verticies.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Generate a vertex array object
        gl::GenVertexArrays(1, &mut vao);
    }

    // Copy the verticies into a buffer
    unsafe {
        // Bind the VAO and VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindVertexArray(vao);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verticies.len() * 4) as isize,
            verticies.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Link vertex attributes
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    println!("VBO: {}", vbo);
    println!("VAO: {}", vao);

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();

        // Draw the triangle
        unsafe {
            gl::UseProgram(shader_program);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

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

    unsafe {
        // Delete the shader program
        gl::DeleteProgram(shader_program);

        // Delete the buffer
        gl::DeleteBuffers(1, &vbo);
    }
}
