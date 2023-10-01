use std::mem::size_of;

use gl::types::GLvoid;

use super::vao::Vao;

/// Stores the number of elements in the layer, and the overall size.
#[derive(Debug)]
struct LayerSize(usize, usize);

/// A struct that is used to build a VAO.
#[derive(Debug)]
pub struct VaoBuilder {
    /// The layers of the VAO.
    layers: Vec<LayerSize>,
}

impl VaoBuilder {
    /// Creates a new VAO builder.
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    /// Adds a layer to the VAO.
    pub fn add_layer<T>(&mut self, n: usize) -> &mut Self {
        self.layers.push(LayerSize(n, size_of::<T>()));
        self
    }

    /// Builds the VAO.
    pub fn build(&self) -> Vao {
        let mut id = 0;

        unsafe {
            // Generate the VAO
            gl::GenVertexArrays(1, &mut id);
            // Bind the VAO
            gl::BindVertexArray(id);

            // The offset of the current layer
            let mut offset = 0;

            // The offset between each vertex
            let stride = self.layers.iter().fold(0, |acc, layer| acc + layer.1);

            // Iterate over the layers
            for (idx, layer) in self.layers.iter().enumerate() {
                // Enable the vertex attribute array
                gl::EnableVertexAttribArray(idx as u32);
                // Set the vertex attribute pointer
                gl::VertexAttribPointer(
                    0,
                    layer.0 as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const GLvoid,
                );

                // Increment the offset
                offset += layer.0;
            }

            // Unbind the VAO
            gl::BindVertexArray(0);
        }

        Vao { id }
    }
}
