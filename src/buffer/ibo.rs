#![allow(dead_code)]
use gl::types::{GLenum, GLsizeiptr, GLvoid};

/// An Index Buffer Object
#[derive(Debug, Clone, Copy)]
pub struct Ibo {
    id: u32,
}

impl Ibo {
    /// Creates a new IBO.
    pub fn new(indicies: &[u32], usage: GLenum) -> Ibo {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        let mut this = Ibo { id };

        this.set_data(indicies, usage);

        this
    }

    /// Binds the IBO.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    /// Unbinds the IBO.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    /// Sets the data of the IBO.
    pub fn set_data(&mut self, data: &[u32], usage: GLenum) {
        unsafe {
            self.bind();
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of_val(data)) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            );
            self.unbind();
        }
    }
}
