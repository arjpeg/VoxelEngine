use std::{ffi::CString, process::exit};

use gl::types::GLchar;
use log::info;
use owo_colors::OwoColorize;

use nalgebra_glm as glm;

use crate::rendering::shader::shader::{Shader, ShaderKind};

pub struct ShaderProgram {
    id: u32,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

pub enum UniformValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
    Mat4(glm::Mat4),
}

#[allow(dead_code)]
impl ShaderProgram {
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
                println!("{}", std::str::from_utf8(info_log).unwrap());
            }

            exit(1);
        }

        info!("Shader program linked successfully!");

        self.id = shader_program;
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub fn set_uniform(&self, name: &str, value: impl Into<UniformValue>) {
        let value = value.into();

        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name.as_ptr());

            match value {
                UniformValue::Bool(value) => {
                    gl::Uniform1i(location, value as i32);
                }
                UniformValue::Int(value) => {
                    gl::Uniform1i(location, value);
                }
                UniformValue::Float(value) => {
                    gl::Uniform1f(location, value);
                }
                UniformValue::Vec3(x, y, z) => {
                    gl::Uniform3f(location, x, y, z);
                }
                UniformValue::Vec4(x, y, z, w) => {
                    gl::Uniform4f(location, x, y, z, w);
                }
                UniformValue::Mat4(mat) => {
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
                }
            }
        }
    }
}

impl Default for ShaderProgram {
    /// Creates a new shader program from the defeault vertex and fragment shaders.
    /// (./res/shaders/vertex.glsl, ./res/shaders/frag.glsl)
    fn default() -> Self {
        Self::new("./assets/shaders/vertex.glsl", "./assets/shaders/frag.glsl")
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl From<bool> for UniformValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for UniformValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<(f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::Vec3(value.0, value.1, value.2)
    }
}

impl From<glm::Vec3> for UniformValue {
    fn from(value: glm::Vec3) -> Self {
        Self::Vec3(value.x, value.y, value.z)
    }
}

impl From<(f32, f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::Vec4(value.0, value.1, value.2, value.3)
    }
}

impl From<glm::Vec4> for UniformValue {
    fn from(value: glm::Vec4) -> Self {
        Self::Vec4(value.x, value.y, value.z, value.w)
    }
}

impl From<glm::Mat4> for UniformValue {
    fn from(value: glm::Mat4) -> Self {
        Self::Mat4(value)
    }
}
