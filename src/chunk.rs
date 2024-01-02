use std::collections::HashMap;

use noise::NoiseFn;

use crate::{
    rendering::mesh::Mesh,
    voxel::{Voxel, VoxelKind},
};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;

/// Represents a section of the world.
#[derive(Debug)]
pub struct Chunk {
    /// The position of the chunk.
    pub position: (i32, i32),

    /// The cubes in the chunk.
    pub blocks: HashMap<(usize, usize, usize), Voxel>,

    /// The mesh of the chunk.
    pub mesh: Option<Mesh>,
}

impl Chunk {
    /// Creates a new chunk.
    pub fn new(position: (i32, i32)) -> Self {
        let mut cubes = HashMap::new();
        cubes.reserve(CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT);

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    let true_x = x as i32 + position.0 * CHUNK_WIDTH as i32;
                    let true_y = y as i32;
                    let true_z = z as i32 + position.1 * CHUNK_WIDTH as i32;

                    // cubes[get_chunk_index((x, y, z))].position = (true_x, true_y, true_z);
                    cubes.insert(
                        (x, y, z),
                        Voxel {
                            position: (true_x, true_y, true_z),
                            kind: VoxelKind::Air,
                        },
                    );
                }
            }
        }

        Self {
            position,
            blocks: cubes,
            mesh: None,
        }
    }
}
