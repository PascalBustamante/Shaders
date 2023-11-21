use gl::types::*;
use crate::VBO;

pub struct VAO {
    // id reference for the Vertex Array Object
    pub id: GLuint,
}

impl VAO {
    // Constructor
    pub fn new() -> Self {
        let mut vao = VAO { id: 0 };
        unsafe {
            gl::GenVertexArrays(1, &mut vao.id);
        }
        vao
    }

    // Bind the VAO
    pub fn bind(&self) {
        unsafe{
            gl::BindVertexArray(self.id);
        }
    }

    // Unbind the VAO
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    // Deletes the VAO
    pub fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }

    // Links a VBO to the VAO using a certain layout
    pub fn link_vbo(&self, vbo: &VBO, layout: GLuint) {
        vbo.bind();
        unsafe{
            gl::VertexAttribPointer(layout, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());  // this might cause errors 
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}
