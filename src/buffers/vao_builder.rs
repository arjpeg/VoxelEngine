use std::mem::size_of;

use gl::types::GLvoid;
use owo_colors::OwoColorize;

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
        // Make sure a VBO is bound
        debug_assert!({
            let mut id = 0;
            unsafe { gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut id) };

            if id == 0 {
                panic!(
                    "{}: No was bound VBO during creation of the vao",
                    "Error".red().bold()
                );
            }

            true
        });

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

            // The offset between each vertex
            let sizes = self
                .layers
                .iter()
                .map(|layer| layer.0 * layer.1)
                .collect::<Vec<_>>();

            let stride = sizes.iter().sum::<usize>();

            // Iterate over the layers
            for (idx, layer) in self.layers.iter().enumerate() {
                // The offset of the layer
                let offset = sizes[..idx].iter().sum::<usize>();

                // Set the vertex attribute pointer
                gl::VertexAttribPointer(
                    idx as u32,
                    layer.0 as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const GLvoid,
                );

                // Enable the vertex attribute array
                gl::EnableVertexAttribArray(idx as u32);
            }

            // Unbind the VAO
            gl::BindVertexArray(0);
        }

        Vao { id }
    }
}
