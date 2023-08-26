use std::{ffi::CString, mem, ptr};

use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};

unsafe fn compile_shader(shader_type: GLenum, source: &str) -> GLuint {
    // Create the vertex shader
    let shader = gl::CreateShader(shader_type);
    let shader_source = CString::new(source.as_bytes()).unwrap();

    // Compile the vertex shader
    gl::ShaderSource(shader, 1, &shader_source.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    // Check for errors
    let mut success: GLint = 1;
    let mut info_log: [GLchar; 512] = [0; 512];

    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != 1 {
        gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr());

        let info_log: &[u8] =
            std::slice::from_raw_parts(info_log.as_ptr() as *const u8, info_log.len());

        panic!(
            "Error while compiling shader: {}",
            std::str::from_utf8(&info_log).unwrap()
        );
    }

    shader
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2)); // Use a version that macOS supports
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

    let verticies: [GLfloat; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
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

    // Compile the shaders
    let shader_program: GLuint;
    let vertex_shader: GLuint;
    let fragment_shader: GLuint;

    unsafe {
        vertex_shader = compile_shader(gl::VERTEX_SHADER, include_str!("../shaders/vertex.glsl"));
        fragment_shader = compile_shader(gl::FRAGMENT_SHADER, include_str!("../shaders/frag.glsl"));

        // Create the shader program
        shader_program = gl::CreateProgram();

        // Attach the shaders to the program
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for errors
        let mut success: GLint = 1;

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != 1 {
            let mut info_log: [GLchar; 512] = [0; 512];

            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr());

            let info_log: &[u8] =
                std::slice::from_raw_parts(info_log.as_ptr() as *const u8, info_log.len());

            panic!(
                "Error while linking shader program: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }
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

        // Delete the shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // Delete the buffer
        gl::DeleteBuffers(1, &vbo);
    }
}
