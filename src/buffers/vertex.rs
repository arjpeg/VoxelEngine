use nalgebra_glm as glm;

///  Represents a vertex that can be used in a vertex buffer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: glm::Vec3,
    /// The color of the vertex.
    pub color: glm::Vec3,
}

impl Vertex {
    /// Creates a new vertex.
    pub fn new(position: glm::Vec3, color: glm::Vec3) -> Self {
        Self { position, color }
    }
}
