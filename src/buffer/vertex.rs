///  Represents a vertex that can be used in a vertex buffer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: [f32; 3],
    /// The color of the vertex.
    pub color: [f32; 3],
    /// The texture coordinates of the vertex.
    pub texture_coords: [f32; 2],
}

impl Vertex {
    /// Creates a new vertex.
    pub fn new(position: [f32; 3], color: [f32; 3], texture_coords: [f32; 2]) -> Vertex {
        Vertex {
            position,
            color,
            texture_coords,
        }
    }
}
