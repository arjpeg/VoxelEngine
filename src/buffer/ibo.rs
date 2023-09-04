#![allow(dead_code)]
use std::mem::size_of;

use gl::types::{GLenum, GLsizeiptr, GLvoid};

/// An Index Buffer Object
pub struct IBO {
    id: u32,
}

impl IBO {
    /// Creates a new IBO.
    pub fn new(indicies: &[u32], usage: GLenum) -> IBO {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        let mut this = IBO { id };

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
                (data.len() * size_of::<u32>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            );
            self.unbind();
        }
    }
}
