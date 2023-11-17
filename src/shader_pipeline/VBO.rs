pub struct VBO {
    // Reference ID for the Vertex Buffer Object
    pub ID: GLuint,
}

impl VBO {
    // Constructor that generates a Vertex Buffer Object and links it to vertices
    pub fn new(vertices: &[gl::FLOAT]) -> Self {
        let mut vbo = VBO { ID: 0 };
        unsafe {
            gl::GenBuffers(1, &mut vbo.ID);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.ID);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (vertices.len() * std::mem::size_of::<GLfloat>()) as gl::isize, 
                vertices.as_ptr() as *const std::ffi::c_void, 
                gl::STATIC_DRAW);
        }
        vbo
    }

    // Binds the VBO
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ID);
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
            gl::DeleteBuffers(1, &self.ID);
        }
    }
}