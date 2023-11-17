pub struct EBO {
    // Reference ID of Elements Buffer Object
    pub ID: GLuint,
}

impl EBO {
    // Constructor that generates a Elements Buffer Object and links it to indices
    pub fn new(indicies: &[GLuint]) {
        let mut ebo = EBO { ID: 0 };
        unsafe {
            gl::GenBuffers(1, &mut ebo.ID);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.ID);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
                (indices.len() * std::mem::size_of::<GLuint>()) as gl::isize,
                indices.as_ptr() as *const std::ffi::c_void, 
                gl::STATIC_DRAW);
        }
        ebo
    }

    // Bind the EBO
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ID);
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
            gl::DeleteBuffers(1, &self.ID);
        }
    }
}