use nalgebra_glm as glm;
use noise::NoiseFn;
use owo_colors::OwoColorize;

use crate::{
    utils::get_chunk_index,
    voxel::{Voxel, VoxelKind},
    NOISE,
};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 32;

/// Represents a section of the world.
#[derive(Debug)]
pub struct Chunk {
    /// The position of the chunk.
    pub position: (i32, i32),

    /// The cubes in the chunk.
    pub cubes: [Voxel; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT],
}

impl Chunk {
    /// Creates a new chunk.
    pub fn new(position: (i32, i32)) -> Self {
        let mut cubes = [Voxel {
            position: glm::vec3(0, 0, 0),
            kind: VoxelKind::Air,
        }; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT];

        println!(
            "Creating chunk at {:?} (with size {})",
            (position).cyan().bold(),
            cubes.len()
        );

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    cubes[get_chunk_index(x, y, z)].position =
                        glm::vec3(x as u32, y as u32, z as u32);
                }
            }
        }

        Self { position, cubes }
    }
}

/// Different strategies for generating chunks.
#[derive(Debug)]
pub enum ChunkGenerationStrategy {
    /// Generates a chunk with all air.
    Empty,
    /// Performs a perlin noise generation for the height of the chunk.
    Perlin2d,
    /// Performs a perlin noise generation in all 3 dimensions.
    Perlin3d,
    /// Places a flat plane of some block below a certain height.
    FlatPlane(VoxelKind, u32),
}

impl ChunkGenerationStrategy {
    /// Populates a chunk with the given strategy.
    /// Takes in the coordinates of the chunk, and the chunk itself.
    pub fn apply(&self, chunk: &mut Chunk) {
        match &self {
            ChunkGenerationStrategy::Empty => {
                // Set all voxels to air
                for voxel in chunk.cubes.iter_mut() {
                    voxel.kind = VoxelKind::Air;
                }
            }
            ChunkGenerationStrategy::Perlin2d => {
                self.perlin_2d(chunk);
            }
            ChunkGenerationStrategy::Perlin3d => {
                self.perlin_3d(chunk);
            }
            ChunkGenerationStrategy::FlatPlane(kind, height) => {
                self.flat_plane(chunk, *kind, *height);
            }
        }
    }

    /// Performs a perlin noise generation in 2 dimensions.
    /// The height of each voxel is determined by the noise value.
    fn perlin_2d(&self, chunk: &mut Chunk) {
        let noise = NOISE.get().unwrap();

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                let noise_value = noise.get([
                    (x + chunk.position.0 as usize) as f64 / 16.0,
                    (z + chunk.position.1 as usize) as f64 / 16.0,
                ]) as f32;

                let height = (noise_value * CHUNK_HEIGHT as f32).max(1.0) as usize;

                for y in 0..height {
                    chunk.cubes[get_chunk_index(x, y as usize, z)].kind = VoxelKind::Grass;
                }
            }
        }
    }

    /// Performs a perlin noise generation in 3 dimensions.
    fn perlin_3d(&self, chunk: &mut Chunk) {
        let noise = NOISE.get().unwrap();

        for voxel in chunk.cubes.iter_mut() {
            let noise_value = noise.get([
                voxel.position.x as f64 / 16.0,
                voxel.position.y as f64 / 16.0,
                voxel.position.z as f64 / 16.0,
            ]) as f32;

            if noise_value > 0.0 {
                voxel.kind = VoxelKind::Grass;
            } else {
                voxel.kind = VoxelKind::Air;
            }
        }
    }

    /// Places a flat plane of some block below a certain height.
    fn flat_plane(&self, chunk: &mut Chunk, kind: VoxelKind, height: u32) {
        for voxel in chunk
            .cubes
            .iter_mut()
            .filter(|voxel| voxel.position.y <= height)
        {
            voxel.kind = kind;
        }
    }
}
