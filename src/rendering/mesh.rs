use crate::{chunk::Chunk, utils::world_to_chunk_coordinate, voxel::VoxelKind};

/// A vertex that can be passed to the GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: (f32, f32, f32),
}

/// A mesh that can be passed to the GPU.
pub struct Mesh {
    /// The vertices of the mesh.
    pub vertices: Vec<Vertex>,
    /// The indices of the mesh.
    pub indices: Vec<u32>,
}

/// The different directions that a face can be facing.
#[derive(Clone, Copy)]
pub enum FaceDirection {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
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
        let position_offsets = (-1..=1)
            .flat_map(move |x| (-1..=1).map(move |y| (x, y)))
            .filter(|(x, y)| *x != 0 || *y != 0);

        // Iterate through each chunk
        for chunk in chunks.iter() {
            let adjacent_chunks = position_offsets
                .clone()
                .map(|(x, y)| (chunk.position.0 + x, chunk.position.1 + y))
                .map(|(x, y)| chunks.iter().find(|chunk| chunk.position == (x, y)))
                .collect::<Vec<_>>();

            // Go through each block
            for voxel in chunk.blocks.iter() {
                // If the voxel is air, skip it
                if voxel.kind == VoxelKind::Air {
                    continue;
                }

                // Get the position of the voxel
                let (x, y, z) = voxel.position;
                let (cx, cy, cz) = world_to_chunk_coordinate(x, y, z);
                // println!("chunk coords: {cx}, {cy}, {cz}");

                // Add the all of the faces
                // if cy < CHUNK_HEIGHT
                //     && chunk.blocks[get_chunk_index((cx, cy + 1, cz))].kind == VoxelKind::Air
                // {
                //     self.add_quad(position, size, FaceDirection::Up);
                // }

                // self.add_quad(position, size, FaceDirection::Up);
                // self.add_quad(position, size, FaceDirection::Down);
                self.add_quad(voxel.position, FaceDirection::Front);
                // self.add_quad(position, size, FaceDirection::Right);
                // self.add_quad(position, size, FaceDirection::Front);
                // self.add_quad(position, size, FaceDirection::Back);
            }
        }

        // Return the mesh
        self.mesh
    }

    /// Adds a quad to the mesh.
    fn add_quad(&mut self, position: (i32, i32, i32), direction: FaceDirection) {
        // Add the indices
        let index_offset = match self.mesh.indices.iter().max() {
            Some(max) => max + 1,
            None => 0,
        };

        self.mesh.indices.push(index_offset + 0);
        self.mesh.indices.push(index_offset + 1);
        self.mesh.indices.push(index_offset + 2);

        self.mesh.indices.push(index_offset + 2);
        self.mesh.indices.push(index_offset + 3);
        self.mesh.indices.push(index_offset + 0);

        // Add the vertices
        let positions = Self::get_face_verticies(position, direction);
        positions.iter().for_each(|position| {
            self.mesh.vertices.push(Vertex {
                position: (position.0 as f32, position.1 as f32, position.2 as f32),
            });
        });
    }

    /// Returns the vertices of the face of a cube based on its position, size, and
    /// direction. All of the verticies move in a counter-clockwise direction.
    #[rustfmt::skip]
    fn get_face_verticies(
        position: (i32, i32, i32),
        direction: FaceDirection,
    ) -> [(i32, i32, i32); 4] {
        let (x, y, z) = position;

        match direction {
            FaceDirection::Up => [
                (position.0, position.1 + 1, position.2),
                (position.0 + 1, position.1 + 1, position.2),
                (position.0 + 1, position.1 + 1, position.2 + 1),
                (position.0, position.1 + 1, position.2 + 1),
            ],
            FaceDirection::Down => [
                (x, y, z),
                (x + 1, y, x),
                (x + 1, y, z + 1),
                (x, y, z + 1),
            ],
            FaceDirection::Left => [
                (position.0, position.1, position.2),
                (position.0, position.1 + 1, position.2),
                (position.0, position.1 + 1, position.2 + 1),
                (position.0, position.1, position.2 + 1),
            ],
            FaceDirection::Right => [
                (position.0 + 1, position.1, position.2),
                (position.0 + 1, position.1 + 1, position.2),
                (position.0 + 1, position.1 + 1, position.2 + 1),
                (position.0 + 1, position.1, position.2 + 1),
            ],
            FaceDirection::Front => [
                (x,   y+1, z),
                (x,   y,   z),
                (x+1, y,   z),
                (x+1, y+1, z),
            ],
            FaceDirection::Back => [
                (position.0, position.1, position.2),
                (position.0 + 1, position.1, position.2),
                (position.0 + 1, position.1 + 1, position.2),
                (position.0, position.1 + 1, position.2),
            ],
        }
    }
}
