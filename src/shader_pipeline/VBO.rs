use gl::types::*;
use std::ffi::c_void;

pub struct VBO {
    // Reference id for the Vertex Buffer Object
    pub id: GLuint,
}

impl VBO {
    // Constructor that generates a Vertex Buffer Object and links it to vertices
    pub fn new(vertices: &[GLfloat]) -> Self {
        let mut vbo = VBO { id: 0 };
        unsafe {
            gl::GenBuffers(1, &mut vbo.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize, 
                vertices.as_ptr() as *const std::ffi::c_void, 
                gl::STATIC_DRAW);
        }
        vbo
    }

    // Binds the VBO
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    // Unbinds the VBO
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    // Deletes the VBO
    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}