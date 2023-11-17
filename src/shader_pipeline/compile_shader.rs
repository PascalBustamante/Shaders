pub fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).expect("CString::new failed");
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check compilation status
        let mut success: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        //println!("inside compiler.");
        if success == gl::FALSE as GLint {
            let mut len: GLint = 0;
            println!("ERROR.");

            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = vec![0; len as usize]; // Initialize buffer with zeros
//            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
//            buffer.extend([b' '].iter().cycle().take(len as usize - 1));
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            let error_message = String::from_utf8_lossy(&buffer);
            panic!("Shader compilation failed: {}", error_message);
        }
        //println!("past eror msg.");

        shader
    }
}