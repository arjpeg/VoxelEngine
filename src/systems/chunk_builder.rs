use crate::{
    chunk::{Chunk, CHUNK_HEIGHT, CHUNK_WIDTH},
    voxel::VoxelKind,
    NOISE,
};

use noise::NoiseFn;

pub const NOISE_SCALE: f64 = 0.01;

/// Different strategies for generating chunks.
#[derive(Debug)]
#[allow(dead_code)]
pub enum ChunkGenStrategy {
    /// Generates a chunk with all air.
    Empty,
    /// Performs a perlin noise generation for the height of the chunk.
    Perlin2d,
    /// Performs a perlin noise generation in all 3 dimensions.
    Perlin3d,
    /// Places a flat plane of some block below a certain height.
    FlatPlane(VoxelKind, u32),
    /// A series of single voxels (used for testing)
    SingleVoxels(Vec<(usize, usize, usize)>),
}

impl ChunkGenStrategy {
    /// Populates a chunk with the given strategy.
    /// Takes in the coordinates of the chunk, and the chunk itself.
    pub fn apply(&self, chunk: &mut Chunk) {
        match &self {
            ChunkGenStrategy::Empty => {
                // Set all voxels to air
                for (_, voxel) in chunk.blocks.iter_mut() {
                    voxel.kind = VoxelKind::Air;
                }
            }
            ChunkGenStrategy::Perlin2d => {
                self.perlin_2d(chunk);
            }
            ChunkGenStrategy::Perlin3d => {
                self.perlin_3d(chunk);
            }
            ChunkGenStrategy::FlatPlane(kind, height) => {
                self.flat_plane(chunk, *kind, *height);
            }
            ChunkGenStrategy::SingleVoxels(voxels) => {
                for (x, y, z) in voxels {
                    chunk.blocks.get_mut(&(*x, *y, *z)).unwrap().kind = VoxelKind::Grass;
                }
            }
        }
    }

    /// Performs a perlin noise generation in 2 dimensions.
    /// The height of each voxel is determined by the noise value.
    fn perlin_2d(&self, chunk: &mut Chunk) {
        let noise = NOISE.get().unwrap();
        let (chunk_x, chunk_z) = chunk.position;

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                let pos = (
                    (x as i32 + chunk_x * CHUNK_WIDTH as i32) as f64 * NOISE_SCALE,
                    (z as i32 + chunk_z * CHUNK_WIDTH as i32) as f64 * NOISE_SCALE,
                );

                let noise_value = noise.get([pos.0, pos.1]) as f32;
                let noise_value = (noise_value + 1.0) / 2.0;
                let noise_value = noise_value.clamp(0.0, 1.0);

                let height = (noise_value * CHUNK_HEIGHT as f32) as usize;

                for y in 1..height {
                    chunk.blocks.get_mut(&(x, y, z)).unwrap().kind = VoxelKind::Grass;
                }
            }
        }
    }

    /// Performs a perlin noise generation in 3 dimensions.
    fn perlin_3d(&self, chunk: &mut Chunk) {
        let noise = NOISE.get().unwrap();

        for (_, voxel) in chunk.blocks.iter_mut() {
            let noise_value = noise.get([
                voxel.position.0 as f64 * NOISE_SCALE,
                voxel.position.1 as f64 * NOISE_SCALE,
                voxel.position.2 as f64 * NOISE_SCALE,
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
        for (_, voxel) in chunk
            .blocks
            .iter_mut()
            .filter(|(_, voxel)| voxel.position.1 <= height as i32)
        {
            voxel.kind = kind;
        }
    }
}
