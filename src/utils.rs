use crate::chunk::{CHUNK_HEIGHT, CHUNK_WIDTH};

/// Returns the index of a block given its position in the chunk.
/// For example, (0, 10, 0) -> 10. Used for indexing into the chunk's
/// `cubes` array.
pub fn get_chunk_index(chunk_coord: (usize, usize, usize)) -> usize {
    let (x, y, z) = chunk_coord;

    x + CHUNK_WIDTH * (y + CHUNK_HEIGHT * z)
}

/// Returns the coordinate of a chunk, given a block's position in the world.
/// For example, (17, 0, 0) -> (1, 0).
pub fn world_to_chunk_position(x: i32, z: i32) -> (i32, i32) {
    (
        x.div_euclid(CHUNK_WIDTH as i32),
        z.div_euclid(CHUNK_WIDTH as i32),
    )
}

/// Return the chunk position of a block given its position in the world.
/// For example, (17, 0, 0) -> (1, 0, 0). Used for indexing into the
/// `chunks` array.
pub fn world_to_chunk_coordinate(x: i32, y: i32, z: i32) -> (usize, usize, usize) {
    (
        x.rem_euclid(CHUNK_WIDTH as i32) as usize,
        y.rem_euclid(CHUNK_HEIGHT as i32) as usize,
        z.rem_euclid(CHUNK_WIDTH as i32) as usize,
    )
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
