use std::marker::PhantomData;

use gl::types::{GLenum, GLsizeiptr, GLvoid};

/// A Vertex Buffer Object
#[derive(Debug, Clone, Copy)]
pub struct Vbo<T: Sized> {
    pub id: u32,
    _marker: PhantomData<T>,
}

impl<T> Vbo<T> {
    /// Creates a new VBO.
    pub fn new(verticies: &[T], usage: GLenum) -> Self {
        let mut id = 0;
        let size = std::mem::size_of_val(verticies);

        unsafe {
            // Generate the VBO
            gl::GenBuffers(1, &mut id);
            // Bind the VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            // Set the VBO data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as GLsizeiptr,
                verticies.as_ptr() as *const GLvoid,
                usage,
            );

            // Unbind the VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self {
            id,
            _marker: PhantomData,
        }
    }

    /// Binds the VBO.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    /// Unbinds the VBO.
    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    /// Updates the VBO data.
    pub fn update(&self, verticies: &[T]) {
        let size = std::mem::size_of_val(verticies);

        unsafe {
            // Bind the VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            // Set the VBO data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as GLsizeiptr,
                verticies.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // Unbind the VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
