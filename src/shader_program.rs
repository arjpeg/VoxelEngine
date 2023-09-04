use std::{ffi::CString, process::exit};

use gl::types::GLchar;
use owo_colors::OwoColorize;

use nalgebra_glm as glm;

use crate::shader::{Shader, ShaderKind};

pub struct ShaderProgram {
    id: u32,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

#[allow(dead_code)]
impl ShaderProgram {
    /// Creates a new shader program from the defeault vertex and fragment shaders.
    /// (./res/shaders/vertex.glsl, ./res/shaders/frag.glsl)
    pub fn load() -> Self {
        Self::new("./res/shaders/vertex.glsl", "./res/shaders/frag.glsl")
    }

    /// Creates a new shader program from the given vertex and fragment shaders.
    pub fn new(
        vertex_shader_path: &'static str,
        fragment_shader_path: &'static str,
    ) -> ShaderProgram {
        let mut program = ShaderProgram {
            id: 0,
            vertex_shader: Shader::new(vertex_shader_path, ShaderKind::Vertex),
            fragment_shader: Shader::new(fragment_shader_path, ShaderKind::Fragment),
        };

        program.compile_all();

        program
    }

    /// Compiles the vertex and fragment shaders and links them to the shader program.
    fn compile_all(&mut self) {
        self.vertex_shader.compile();
        self.fragment_shader.compile();

        let shader_program = unsafe { gl::CreateProgram() };

        self.vertex_shader.attach(shader_program);
        self.fragment_shader.attach(shader_program);

        unsafe {
            gl::LinkProgram(shader_program);
        }

        // Check for errors
        let mut success = 1;
        let mut info_log: [GLchar; 512] = [0; 512];

        unsafe {
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        }

        if success != 1 {
            unsafe {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr(),
                );

                let info_log: &[u8] =
                    std::slice::from_raw_parts(info_log.as_ptr() as *const u8, info_log.len());

                println!("{} while linking shader program:", "Error".red().bold());
                println!("{}", std::str::from_utf8(&info_log).unwrap());
            }

            exit(1);
        }

        println!("{}", "Shader program linked successfully!".green().bold(),);

        self.id = shader_program;
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let name = CString::new(name).unwrap();

            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let name = CString::new(name).unwrap();

            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let name = CString::new(name).unwrap();

            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            let name = CString::new(name).unwrap();

            gl::Uniform4f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z, w);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: &glm::Mat4) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name.as_ptr());

            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
