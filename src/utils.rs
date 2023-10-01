use crate::chunk::{CHUNK_HEIGHT, CHUNK_WIDTH};

/// Returns the index of a block given its position in the chunk.
/// For example, (0, 10, 0) -> 10. Used for indexing into the chunk's
/// `cubes` array.
pub fn get_chunk_index(x: usize, y: usize, z: usize) -> usize {
    x + CHUNK_WIDTH * (y + CHUNK_HEIGHT * z)
}

/// Returns true if the key is down.
pub fn key_is_down(window: &glfw::Window, key: glfw::Key) -> bool {
    window.get_key(key) == glfw::Action::Press
}

/// Checks for OpenGL errors. If there are any, prints them to the console.
#[macro_export]
macro_rules! get_gl_error {
    ($fn_name:expr) => {
        debug_assert!({
            #[allow(unused_unsafe)]
            unsafe {
                let error = gl::GetError();
                if error != gl::NO_ERROR {
                    println!(
                        "OpenGL error at marker '{}' (error code: {} - {})",
                        $fn_name,
                        error,
                        match error {
                            gl::INVALID_ENUM => "INVALID_ENUM",
                            gl::INVALID_VALUE => "INVALID_VALUE",
                            gl::INVALID_OPERATION => "INVALID_OPERATION",
                            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                            gl::CONTEXT_LOST => "CONTEXT_LOST",
                            _ => "UNKNOWN",
                        }
                    );
                    println!("{}:{}\n", file!(), line!());

                    panic!("OpenGL error");
                }

                true
            }
        })
    };
}
