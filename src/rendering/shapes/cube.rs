use std::mem::size_of;

use crate::buffers::vao::Vao;
use crate::buffers::vbo::Vbo;
use crate::buffers::vertex::Vertex;

use gl::types::GLfloat;
use nalgebra_glm as glm;

/// Represents an equilateral cube.
pub struct Cube {
    /// The model matrix of the cube.
    pub model_matrix: glm::Mat4,

    /// The position of the cube.
    pub position: glm::Vec3,
    /// The cube's color.
    pub color: glm::Vec3,

    /// The vertices of the cube.
    pub vertices: [Vertex; 36],

    /// The vbo object of the cube.
    pub vbo: Vbo,
    /// The vao object of the cube.
    pub vao: Vao,
}

#[allow(dead_code)]
impl Cube {
    /// Creates a new cube.
    pub fn new(position: glm::Vec3) -> Self {
        Self::new_with_color(position, glm::vec3(1.0, 1.0, 1.0))
    }

    /// Creates a new cube with a color.
    pub fn new_with_color(position: glm::Vec3, color: glm::Vec3) -> Self {
        let verticies = Self::get_verticies(color);

        let vbo = Vbo::new(&verticies, gl::STATIC_DRAW);
        vbo.bind();

        let mut vao = Vao::new();
        vao.set_attribute(0, 3, gl::FLOAT, false, 6 * size_of::<GLfloat>(), 0);

        vao.set_attribute(
            1,
            3,
            gl::FLOAT,
            false,
            6 * size_of::<GLfloat>(),
            3 * size_of::<GLfloat>(),
        );

        Self {
            position,
            color,
            model_matrix: glm::translate(&glm::identity(), &position),
            vertices: Self::get_verticies(color),
            vbo,
            vao,
        }
    }

    /// Returns the vertices of the cube (generic)
    fn get_verticies(color: glm::Vec3) -> [Vertex; 36] {
        [
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, -0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(glm::vec3(-0.5, 0.5, -0.5), color),
        ]
    }
}
