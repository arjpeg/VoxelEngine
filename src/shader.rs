use std::{ffi::CString, fs, process::exit, ptr};

use gl::types::GLchar;

use owo_colors::OwoColorize;

pub struct Shader {
    id: u32,
    kind: ShaderKind,
    path: &'static str,
}

pub enum ShaderKind {
    Vertex,
    Fragment,
}

impl Shader {
    pub fn new(path: &'static str, kind: ShaderKind) -> Shader {
        Shader { id: 0, path, kind }
    }

    pub fn compile(&mut self) {
        unsafe {
            self.id = gl::CreateShader(match self.kind {
                ShaderKind::Vertex => gl::VERTEX_SHADER,
                ShaderKind::Fragment => gl::FRAGMENT_SHADER,
            });

            let source = fs::read_to_string(self.path)
                .expect(format!("Failed to read shader file, {}", self.path).as_str());

            let source = CString::new(source.as_bytes()).unwrap();

            // Compile the shader
            gl::ShaderSource(self.id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(self.id);

            // Check for errors
            let mut success = 1;
            let mut info_log: [GLchar; 512] = [0; 512];

            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);

            if success != 1 {
                gl::GetShaderInfoLog(self.id, 512, ptr::null_mut(), info_log.as_mut_ptr());

                let info_log: &[u8] =
                    std::slice::from_raw_parts(info_log.as_ptr() as *const u8, info_log.len());

                println!(
                    "{} while compiling shader '{}':",
                    "Error".red(),
                    self.path.bold()
                );
                println!("{}", std::str::from_utf8(&info_log).unwrap());

                exit(1)
            }
        }
    }

    pub fn attach(&self, program: u32) {
        println!("Attaching shader '{}'", self.path.bold());
        unsafe {
            gl::AttachShader(program, self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
