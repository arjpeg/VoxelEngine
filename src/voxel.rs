use nalgebra_glm as glm;

/// Represents a voxel in the world.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Voxel {
    /// The position of the voxel.
    pub position: glm::UVec3,

    /// The kind of voxel.
    pub kind: VoxelKind,
}

/// The types of voxels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoxelKind {
    /// Air (empty space).
    Air,

    /// Grass
    Grass,
}
