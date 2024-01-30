use std::ops::{Index, IndexMut};

use crate::{
    chunk::{CHUNK_HEIGHT, CHUNK_WIDTH},
    utils::get_chunk_index,
    voxel::{Voxel, VoxelKind},
};

type BlockIndex = (usize, usize, usize);

/// Represents all of the voxels in a chunk.
#[derive(Debug)]
pub struct BlockCollection {
    /// The actual voxel data.
    voxels: Vec<Voxel>,
}

impl BlockCollection {
    /// Creates a new block collection.
    pub fn new() -> Self {
        Self {
            voxels: {
                // Pre-allocate the voxel vector
                let mut voxels = Vec::new();
                voxels.reserve(CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_WIDTH);
                voxels
            },
        }
    }

    /// Creates a new block collection, and fills it with the given voxel kind.
    pub fn new_filled(chunk_offset: (i32, i32), voxel: VoxelKind) -> Self {
        Self {
            voxels: {
                // Pre-allocate the voxel vector
                let mut voxels = Vec::new();
                voxels.reserve(CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_WIDTH);

                // Fill the voxel vector with the given voxel kind
                for x in 0..CHUNK_WIDTH {
                    for z in 0..CHUNK_WIDTH {
                        for y in 0..CHUNK_HEIGHT {
                            let x = x as i32 + chunk_offset.0 * CHUNK_WIDTH as i32;
                            let z = z as i32 + chunk_offset.1 * CHUNK_WIDTH as i32;
                            let y = y as i32;

                            voxels.push(Voxel {
                                position: (x, y, z),
                                kind: voxel,
                            });
                        }
                    }
                }

                voxels
            },
        }
    }

    /// Gets the voxel at the given position.
    pub fn get(&self, pos: BlockIndex) -> Option<&Voxel> {
        self.voxels.get(get_chunk_index(pos))
    }

    /// Gets the voxel given the chunk-block index.
    /// Panics if the index is out of bounds (i.e. greater than `CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_WIDTH`).
    pub fn get_by_index(&self, index: usize) -> &Voxel {
        &self.voxels[index]
    }
}

impl Index<BlockIndex> for BlockCollection {
    type Output = Voxel;

    fn index(&self, index: BlockIndex) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<BlockIndex> for BlockCollection {
    fn index_mut(&mut self, index: BlockIndex) -> &mut Self::Output {
        self.voxels.index_mut(get_chunk_index(index))
    }
}
