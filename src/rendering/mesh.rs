use crate::{chunk::Chunk, voxel::VoxelKind};

/// A mesh that can be passed to the GPU.
pub struct Mesh {
    /// The vertices of the mesh.
    pub vertices: Vec<(f32, f32, f32)>,
    /// The indices of the mesh.
    pub indices: Vec<u32>,
}

/// A struct that builds a mesh from a set of voxels.
pub struct MeshBuilder {
    /// The mesh that is being built.
    mesh: Mesh,
}

impl MeshBuilder {
    /// Creates a new mesh builder.
    pub fn new() -> Self {
        Self {
            mesh: Mesh {
                vertices: Vec::new(),
                indices: Vec::new(),
            },
        }
    }

    /// Builds the mesh from a list of chunks
    pub fn build_mesh(mut self, chunks: &Vec<Chunk>) -> Mesh {
        // Iterate through each chunk
        for chunk in chunks.iter() {
            // Go through each block
            for voxel in chunk.blocks.iter() {
                // If the voxel is air, skip it
                if voxel.kind == VoxelKind::Air {
                    continue;
                }

                // Get the position of the voxel
                let position = (
                    voxel.position.0 as f32,
                    voxel.position.1 as f32,
                    voxel.position.2 as f32,
                );

                // Get the size of the voxel
                let size = (1.0, 1.0);

                // Add the quad to the mesh
                self.add_quad(position, size);
            }
        }

        // Return the mesh
        self.mesh
    }

    /// Adds a quad to the mesh.
    fn add_quad(&mut self, position: (f32, f32, f32), size: (f32, f32)) {
        // Add the indices
        let index_offset = self.mesh.indices.len() as u32;

        self.mesh.indices.push(index_offset + 0);
        self.mesh.indices.push(index_offset + 1);
        self.mesh.indices.push(index_offset + 2);

        self.mesh.indices.push(index_offset + 2);
        self.mesh.indices.push(index_offset + 3);
        self.mesh.indices.push(index_offset + 0);

        // Add the vertices
        self.mesh
            .vertices
            .push((position.0, position.1, position.2));
        self.mesh
            .vertices
            .push((position.0 + size.0, position.1, position.2));
        self.mesh
            .vertices
            .push((position.0 + size.0, position.1 + size.1, position.2));
        self.mesh
            .vertices
            .push((position.0, position.1 + size.1, position.2));
    }
}
