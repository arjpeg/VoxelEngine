use nalgebra_glm as glm;

use crate::{
    utils::get_index,
    voxel::{Voxel, VoxelKind},
};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;

/// Represents a section of the world.
#[derive(Debug)]
pub struct Chunk {
    /// The position of the chunk.
    pub position: glm::Vec2,

    /// The cubes in the chunk.
    pub cubes: [Voxel; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT],
}

impl Chunk {
    /// Creates a new chunk.
    pub fn new(position: glm::Vec2) -> Self {
        let mut cubes = [Voxel {
            position: glm::vec3(0, 0, 0),
            kind: VoxelKind::Air,
        }; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT];

        println!(
            "Creating chunk at {:?} (with size {})",
            position,
            cubes.len()
        );

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    cubes[get_index(x, y, z)] = Voxel {
                        position: glm::vec3(x as u32, y as u32, z as u32),
                        kind: VoxelKind::Grass,
                    };
                }
            }
        }

        Self { position, cubes }
    }
}
