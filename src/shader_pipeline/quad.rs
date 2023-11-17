// Function to render a quad with the given shader and texture
fn render_quad(program: GLuint, texture_id: GLuint, vao: GLuint) {
    //println!("Inside quad.");

    unsafe {
        // Use the shader program
        gl::UseProgram(program);

        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("OpenGL error: {}", error);
        }

        // Activate a texture unit
        gl::ActiveTexture(gl::TEXTURE0);

        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("OpenGL error: {}", error);
        }

        // Bind the texture to the active texture unit
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("OpenGL error: {}", error);
        }

        // Get the location of the 'inputTexture' uniform in the shader
        let texture_uniform_location = gl::GetUniformLocation(program, "inputTexture\0".as_ptr() as *const i8);

        // Set the texture unit index as the value for the 'inputTexture' uniform
        gl::Uniform1i(texture_uniform_location, 0);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("OpenGL error: {}", error);
        }

        // Render the quad
        gl::BindVertexArray(vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("OpenGL error: {}", error);
        }
    }
    println!("After gl stuuf.");

}
