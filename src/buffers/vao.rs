/// Represents a Vertex Array Object (VAO) in OpenGL.
#[derive(Debug, Clone, Copy)]
pub struct Vao {
    id: u32,
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_attribute(
        &mut self,
        index: u32,
        size: i32,
        ty: gl::types::GLenum,
        normalized: bool,
        stride: usize,
        offset: usize,
    ) {
        unsafe {
            self.bind();
            gl::VertexAttribPointer(
                index,
                size,
                ty,
                normalized as gl::types::GLboolean,
                stride as gl::types::GLsizei,
                offset as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(index);
            self.unbind();
        }
    }
}
