use gl::types::*;
use std::ffi::c_void;

pub struct EBO {
    // Reference id of Elements Buffer Object
    pub id: GLuint,
}

impl EBO {
    // Constructor that generates a Elements Buffer Object and links it to indices
    pub fn new(indicies: &[GLuint]) -> Self {
        let mut ebo = EBO { id: 0 };
        unsafe {
            gl::GenBuffers(1, &mut ebo.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
                (indicies.len() * std::mem::size_of::<GLuint>()) as isize,
                indicies.as_ptr() as *const std::ffi::c_void, 
                gl::STATIC_DRAW);
        }
        ebo
    }

    // Bind the EBO
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    // Unbind the EBO
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    // Deletes the EBO
    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}