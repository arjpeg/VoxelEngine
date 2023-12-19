use crate::{
    chunk::Chunk,
    utils::{world_to_chunk_coordinate, world_to_chunk_position},
    voxel::VoxelKind,
};

/// A vertex that can be passed to the GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: (f32, f32, f32),
    /// The normal of the vertex.
    pub normal: (f32, f32, f32),
}

/// A mesh that can be passed to the GPU.
#[derive(Clone, Debug)]
pub struct Mesh {
    /// The vertices of the mesh.
    pub vertices: Vec<Vertex>,
    /// The indices of the mesh.
    pub indices: Vec<u32>,
}

/// The different directions that a face can be facing.
#[derive(Debug, Clone, Copy)]
pub enum FaceDirection {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl FaceDirection {
    /// Returns a list of all of the directions.
    pub fn all() -> [FaceDirection; 6] {
        [
            FaceDirection::Up,
            FaceDirection::Down,
            FaceDirection::Left,
            FaceDirection::Right,
            FaceDirection::Front,
            FaceDirection::Back,
        ]
    }

    /// Returns the face normal of the direction.
    pub const fn normal(&self) -> (f32, f32, f32) {
        match self {
            FaceDirection::Up => (0.0, 1.0, 0.0),
            FaceDirection::Down => (0.0, -1.0, 0.0),
            FaceDirection::Left => (-1.0, 0.0, 0.0),
            FaceDirection::Right => (1.0, 0.0, 0.0),
            FaceDirection::Front => (0.0, 0.0, -1.0),
            FaceDirection::Back => (0.0, 0.0, 1.0),
        }
    }
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
    pub fn build_mesh(mut self, chunks: &[Chunk]) -> Mesh {
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
            println!("Building chunk: {:?}", chunk.position);

            // Go through each block
            self.build_chunk_mesh(chunk, &adjacent_chunks);
        }

        self.mesh
    }

    /// Builds the mesh for a single chunk.
    pub fn build_chunk_mesh(&mut self, chunk: &Chunk, adjacent_chunks: &[Option<&Chunk>]) {
        // Go through each block
        for (_, voxel) in chunk.blocks.iter() {
            // If the voxel is air, skip it
            if voxel.kind == VoxelKind::Air {
                continue;
            }

            // Add all faces that are not adjacent to another voxel
            for direction in FaceDirection::all().iter() {
                self.add_quad_if_not_adjacent(voxel.position, *direction, chunk, adjacent_chunks);
            }
        }
    }

    /// Checks if a voxel is adjacent to another voxel, in a given direction,
    /// and if not, adds a quad to the mesh.
    fn add_quad_if_not_adjacent(
        &mut self,
        position: (i32, i32, i32),
        direction: FaceDirection,
        chunk: &Chunk,
        adjacent_chunks: &[Option<&Chunk>],
    ) {
        if !self.is_adjacent(position, direction, chunk, adjacent_chunks) {
            self.add_quad(position, direction);
        }
    }

    /// Checks if a voxel is adjacent to another voxel, in a given direction.
    fn is_adjacent(
        &self,
        position: (i32, i32, i32),
        direction: FaceDirection,
        chunk: &Chunk,
        adjacent_chunks: &[Option<&Chunk>],
    ) -> bool {
        let (x, y, z) = position;

        let (bx, by, bz) = match direction {
            FaceDirection::Up => (x, y + 1, z),
            FaceDirection::Down => (x, y - 1, z),
            FaceDirection::Left => (x - 1, y, z),
            FaceDirection::Right => (x + 1, y, z),
            FaceDirection::Front => (x, y, z - 1),
            FaceDirection::Back => (x, y, z + 1),
        };

        let this_chunk = world_to_chunk_position(x, z);
        let other_chunk = world_to_chunk_position(bx, bz);

        let chunk_coords = world_to_chunk_coordinate(bx, by, bz);

        let chunk = if this_chunk == other_chunk {
            chunk
        } else {
            match adjacent_chunks
                .iter()
                .find(|chunk| matches!(chunk, Some(chunk) if chunk.position == this_chunk))
                .cloned()
                .flatten()
            {
                Some(chunk) => chunk,
                None => return false,
            }
        };

        // Check if the block exists
        chunk.blocks[&chunk_coords].kind != VoxelKind::Air
    }

    /// Adds a quad to the mesh.
    fn add_quad(&mut self, position: (i32, i32, i32), direction: FaceDirection) {
        // Add the indices
        let index_offset = match self.mesh.indices.last() {
            Some(last) => *last + 4,
            None => 0,
        };

        self.mesh.indices.push(index_offset);
        self.mesh.indices.push(index_offset + 1);
        self.mesh.indices.push(index_offset + 2);

        self.mesh.indices.push(index_offset + 2);
        self.mesh.indices.push(index_offset + 3);
        self.mesh.indices.push(index_offset);

        // Add the vertices
        let verticies = Self::get_face_verticies(position, direction);
        let normal = direction.normal();

        for vertex in verticies.iter() {
            self.mesh.vertices.push(Vertex {
                position: (vertex.0 as f32, vertex.1 as f32, vertex.2 as f32),
                normal,
            });
        }

        // .iter()
        // .for_each(|(x, y, z)| {
        //     dbg!((direction, x - position.0, y - position.1, z - position.2));
        //     std::io::stdin().read_line(&mut String::new()).unwrap();

        //     // self.mesh.vertices.push(Vertex {
        //     //     position: (position.0 as f32, position.1 as f32, position.2 as f32),
        //     // });
        // });
    }

    /// Returns the vertices of the face of a cube based on its position, size, and
    /// direction, including its normals. All of the verticies move in a counter-clockwise direction.
    #[rustfmt::skip]
    pub const fn get_face_verticies(
        position: (i32, i32, i32),
        direction: FaceDirection,
    ) -> [(i32, i32, i32); 4] {
        let (x, y, z) = position;

        match direction {
            FaceDirection::Up => 
                [
                    (x,     y + 1, z + 1),
                    (x,     y + 1, z),
                    (x + 1, y + 1, z),
                    (x + 1, y + 1, z + 1),
                ],
            FaceDirection::Down => 
                [
                    (x,     y, z + 1),
                    (x,     y, z),
                    (x + 1, y, z),
                    (x + 1, y, z + 1),
                ],
            FaceDirection::Left => 
                [
                    (x, y + 1, z + 1),
                    (x, y,     z + 1),
                    (x, y,     z),
                    (x, y + 1, z),
                ],
            FaceDirection::Right => 
                [
                    (x + 1, y + 1, z + 1),
                    (x + 1, y,     z + 1),
                    (x + 1, y,     z),
                    (x + 1, y + 1, z),
                ],
            FaceDirection::Front => 
                [
                    (x,   y+1, z),
                    (x,   y,   z),
                    (x+1, y,   z),
                    (x+1, y+1, z),
                ],
            FaceDirection::Back => 
                [
                    (x,   y+1, z + 1),
                    (x,   y,   z + 1),
                    (x+1, y,   z + 1),
                    (x+1, y+1, z + 1),
                ],
        }
    }
}
