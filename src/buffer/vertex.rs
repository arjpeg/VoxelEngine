///  Represents a vertex that can be used in a vertex buffer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: [f32; 3],
    /// The color of the vertex.
    pub color: [f32; 3],
}

impl Vertex {
    /// Creates a new vertex.
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}
