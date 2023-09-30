use nalgebra_glm as glm;

const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;

/// Represents a section of the world.
#[derive(Debug)]
pub struct Chunk {
    /// The position of the chunk.
    pub position: glm::Vec2,

    /// The cubes in the chunk.
    pub cubes: [glm::UVec3; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT],
}

fn get_index(x: usize, y: usize, z: usize) -> usize {
    x + CHUNK_WIDTH * (y + CHUNK_HEIGHT * z)
}

impl Chunk {
    /// Creates a new chunk.
    pub fn new(position: glm::Vec2) -> Self {
        let mut cubes = [glm::UVec3::zeros(); CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT];

        println!(
            "Creating chunk at {:?} (with size {})",
            position,
            cubes.len()
        );

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    if rand::random::<f32>() < 0.9 {
                        continue;
                    }

                    cubes[get_index(x, y, z)] = glm::vec3(x as u32, y as u32, z as u32);
                }
            }
        }

        Self { position, cubes }
    }
}
