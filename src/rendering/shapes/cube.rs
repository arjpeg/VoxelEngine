use nalgebra_glm as glm;

use crate::{
    buffers::{ibo::Ibo, vao_builder::VaoBuilder, vbo::Vbo},
    get_gl_error,
    rendering::mesh::{FaceDirection, Mesh, Vertex},
};

#[allow(dead_code)]
pub const CUBE_FACES: [(FaceDirection, [(i32, i32, i32); 4]); 6] = {
    [
        (
            FaceDirection::Up,
            [(0, 1, 1), (0, 1, 0), (1, 1, 0), (1, 1, 1)],
        ),
        (
            FaceDirection::Down,
            [(0, 0, 1), (0, 0, 0), (1, 0, 0), (1, 0, 1)],
        ),
        (
            FaceDirection::Left,
            [(0, 1, 1), (0, 0, 1), (0, 0, 0), (0, 1, 0)],
        ),
        (
            FaceDirection::Right,
            [(1, 1, 1), (1, 0, 1), (1, 0, 0), (1, 1, 0)],
        ),
        (
            FaceDirection::Front,
            [(0, 1, 0), (0, 0, 0), (1, 0, 0), (1, 1, 0)],
        ),
        (
            FaceDirection::Back,
            [(0, 1, 1), (0, 0, 1), (1, 0, 1), (1, 1, 1)],
        ),
    ]
};

#[allow(dead_code)]
pub const CUBE_INDICIES: [[u32; 3]; 12] = [
    [0, 1, 2],
    [2, 3, 0],
    [4, 5, 6],
    [6, 7, 4],
    [8, 9, 10],
    [10, 11, 8],
    [12, 13, 14],
    [14, 15, 12],
    [16, 17, 18],
    [18, 19, 16],
    [20, 21, 22],
    [22, 23, 20],
];

#[derive(Debug, Clone)]
pub struct Cube {
    pub position: glm::Vec3,
    pub mesh: Option<Mesh>,
}

#[allow(dead_code)]
impl Cube {
    pub fn new(position: glm::Vec3) -> Self {
        Self {
            position,
            mesh: None,
        }
    }

    pub fn generate_mesh(&mut self) {
        let indices = CUBE_INDICIES.iter().flatten().cloned().collect::<Vec<_>>();
        let mut verticies = Vec::new();

        for (face, positions) in CUBE_FACES {
            let normal = face.normal();

            for position in positions {
                let position = glm::vec3(position.0 as f32, position.1 as f32, position.2 as f32)
                    + self.position;

                verticies.push(Vertex {
                    position: (position.x, position.y, position.z),
                    normal,
                });
            }
        }

        let vbo = Vbo::new(&verticies, gl::STATIC_DRAW);
        vbo.bind();

        get_gl_error!("Cube VBO");

        let vao = VaoBuilder::new()
            .add_layer::<f32>(3)
            .add_layer::<f32>(3)
            .build();

        get_gl_error!("Cube VAO");

        let ibo = Ibo::new(&indices, gl::STATIC_DRAW);

        assert!(indices.len() % 3 == 0);
        get_gl_error!("Cube IBO");

        let mesh = Mesh {
            vertices: verticies,
            indices: indices,

            vao: Some(vao),
            vbo: Some(vbo),
            ibo: Some(ibo),
        };

        self.mesh = Some(mesh);
    }

    pub fn update_vbo(&mut self) {
        let mut verticies = Vec::new();

        for (face, positions) in CUBE_FACES {
            let normal = face.normal();

            for position in positions {
                let position = glm::vec3(position.0 as f32, position.1 as f32, position.2 as f32)
                    + self.position;

                verticies.push(Vertex {
                    position: (position.x, position.y, position.z),
                    normal,
                });
            }
        }

        if let Some(mesh) = self.mesh.as_mut() {
            // Update the VBO
            mesh.vbo.unwrap().bind();
            mesh.vbo.unwrap().update(&verticies);

            mesh.vertices = verticies;
        }
    }
}
