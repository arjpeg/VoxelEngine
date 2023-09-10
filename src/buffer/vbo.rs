use gl::types::{GLenum, GLsizeiptr, GLvoid};

use super::vertex::Vertex;

/// A Vertex Buffer Object
pub struct Vbo {
    id: u32,
}

impl Vbo {
    /// Creates a new VBO.
    pub fn new(verticies: &[Vertex], usage: GLenum) -> Vbo {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        let mut this = Vbo { id };
        this.set_data(verticies, usage);

        this
    }

    /// Binds the VBO.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    /// Unbinds the VBO.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    /// Sets the data of the VBO.
    pub fn set_data(&mut self, data: &[Vertex], usage: GLenum) {
        unsafe {
            self.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of_val(data)) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            );
            self.unbind();
        }
    }
}
