pub struct VAO {
    // ID reference for the Vertex Array Object
    pub ID: GLuint,
}

impl VAO {
    // Constructor
    pub fn new() -> Self {
        let mut vao = VAO { ID: 0 };
        unsafe {
            gl::GenVertexArrays(1, &mut vao.ID);
        }
        vao
    }

    // Bind the VAO
    pub fn bind(&self) {
        unsafe{
            gl::BindVertexArray(self.ID);
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
            gl::DeleteVertexArrays(1, &self.ID);
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
