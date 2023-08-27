mod image;
mod shader;
mod shader_program;

use shader_program::ShaderProgram;
use std::{
    mem::{self, size_of},
    ptr,
};

use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};

use crate::image::Image;

fn load_shaders() -> ShaderProgram {
    let shader_program = ShaderProgram::new("./shaders/vertex.glsl", "./shaders/frag.glsl");

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

    #[rustfmt::skip]
    let verticies: [GLfloat; 32] = [
        // positions      // colors        // texture coords
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // 0 top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // 1 bottom right
       -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // 2 bottom left
       -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // 3 top left 
    ];

    let indicies = [
        1, 0, 2, // first triangle
        0, 3, 2, // second triangle
    ];

    let mut ibo: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;

    let image = Image::new("./res/image/wall.jpg");
    image.bind();

    // Generate buffers
    unsafe {
        // Generates one buffer and stores its id in vbo
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ibo);
        gl::GenVertexArrays(1, &mut vao);
    }

    // Bind and fill the buffers
    unsafe {
        // Bind the VAO
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindVertexArray(vao);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verticies.len() * size_of::<GLfloat>()) as GLsizeiptr,
            verticies.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Link vertex attributes

        // Position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<GLfloat>()) as GLsizei,
            0 as *const GLvoid,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            (6 * mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);

        // Bind the IBO
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indicies.len() * size_of::<GLuint>()) as GLsizeiptr,
            indicies.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    println!("VBO: {}", vbo);
    println!("VAO: {}", vao);
    println!("EBO: {}", ibo);

    unsafe {
        // Turn on wireframe mode
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

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

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, image.id);

            shader_program.set_float("u_time", time_value);

            gl::BindVertexArray(vao);
            gl::BindTexture(gl::TEXTURE_2D, image.id);

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
                _ => {}
            }
        }
    }
}
