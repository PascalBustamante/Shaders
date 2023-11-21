pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Check linking status
        let mut success: GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        println!("inside linkage.");
        if success == gl::FALSE as GLint {
            println!("Linkage broke!");
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize - 1));
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            let error_message = String::from_utf8_lossy(&buffer);
            panic!("Shader program linking failed: {}", error_message);
        }
        //println!("Past error.");

        program
    }
}
