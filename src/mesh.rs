use crate::voxel::{Voxel, VoxelKind};

/// A mesh that can be passed to the GPU.
pub struct Mesh {
    /// The vertices of the mesh.
    pub vertices: Vec<(f32, f32, f32)>,
    /// The indices of the mesh.
    pub indices: Vec<u32>,
}

/// A struct that builds a mesh from a set of voxels.
pub struct MeshBuilder {
    /// The voxels to build the mesh from.
    voxel_data: Vec<Voxel>,
    /// The mesh that is being built.
    mesh: Mesh,
}

impl MeshBuilder {
    /// Creates a new mesh builder.
    pub fn new(voxel_data: Vec<Voxel>) -> Self {
        Self {
            voxel_data,
            mesh: Mesh {
                vertices: Vec::new(),
                indices: Vec::new(),
            },
        }
    }

    /// Builds the mesh.
    pub fn build_mesh(mut self) -> Mesh {
        let position_offsets: [(i32, i32, i32); 6] = [
            (1, 0, 0),  // Right
            (-1, 0, 0), // Left
            (0, 1, 0),  // Top
            (0, -1, 0), // Bottom
            (0, 0, 1),  // Front
            (0, 0, -1), // Back
        ];

        let voxels = self.voxel_data.clone();

        // Loop through all the voxels
        for voxel in voxels.iter() {
            // Skip air voxels
            if voxel.kind == VoxelKind::Air {
                continue;
            }

            // Loop through all the faces
            for face_offset in position_offsets {
                let face = (
                    voxel.position.x + face_offset.0,
                    voxel.position.y + face_offset.1,
                    voxel.position.z + face_offset.2,
                );

                // Check if the face is exposed
                let exposed = self.voxel_data.iter().any(|voxel| {
                    voxel.position.x == face.0
                        && voxel.position.y == face.1
                        && voxel.position.z == face.2
                        && voxel.kind == VoxelKind::Air
                });

                // If the face is exposed, add it to the mesh
                if exposed {
                    let position = (
                        voxel.position.x as f32,
                        voxel.position.y as f32,
                        voxel.position.z as f32,
                    );

                    let size = (1.0, 1.0);

                    self.add_quad(position, size);
                }
            }
        }

        self.mesh
    }

    /// Adds a quad to the mesh.
    fn add_quad(&mut self, position: (f32, f32, f32), size: (f32, f32)) {
        // Add the indices
        let index_offset = self.mesh.vertices.len() as u32;

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
