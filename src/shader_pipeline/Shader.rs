// Read file contents into a String
fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

// Check shader compilation errors
fn check_shader_compile_errors(shader: GLuint, shader_type: &str) {
    let mut success = gl::FALSE as GLint;
    unsafe {
        /*
        When calling C functions that populate a buffer with a null-terminated string, 
        Rust needs to provide a buffer with enough space for the string content plus the null terminator '\0'.
        */
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLuint {
            gl::GetShaderInfoLog(
                shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar
            );

            let error_message = String::from_utf8_lossy(&info_log);
            println!(
                "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}",
                shader_type, error_message
            );
        }
    }
}

// Check shader program linking errors
fn check_program_link_errors(program: GLuint) {
    let mut success = gl::FALSE as GLint;
    unsafe {
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar
            );

            let error_message = String::from_utf8_lossy(&info_log);
            println!("ERROR::PROGRAM_LINKING_ERROR\n{}", error_message);
        }
    }
}

pub struct Shader {
    // Reference ID of the Shader Program
    pub ID: GLuint,
}

impl Shader {
    // Constructor that builds the Shader Program from vertex and fragment shaders
    pub fn new(vertex_file: &str, fragment_file: &str) -> Result<Self, String> {
        // Read the vertex and fragment files and store them as strings
        let vertex_code = read_file_contents(vertex_file).map_err(|e| e.to_string())?;
        let fragment_code = read_file_contents(fragment_file).map_err(|e| e.to_string())?;

        // Convert the shader source strings into CString
        let vertex_source = CString::new(vertex_code).expect("CString conversion failed");
        let fragment_source = CString::new(fragment_code).expect("CString conversion failed");

        // Create Vertex Shader Object and get its reference
        let vertex_shader = unsafe {
            let shader = gl::CreateShader(gl::VERTEX_SHADER);
            // Attach Vertex Shader source to the Vertex Shader Object
            gl::ShaderSource(shader, 1, &vertex_source.as_ptr(), std::ptr::null());
            // Compile the Vertex Shader into machine code
            gl::CompileShader(shader);
            // Check for compilation errors
            check_shader_compile_errors(shader, "VERTEX");
            shader
        };

        // Create Fragment Shader Object and get its reference
        let fragment_shader = unsafe {
            let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            // Attach Fragment Shader source to the Fragment Shader Object
            gl::ShaderSource(shader, 1, &fragment_source.as_ptr(), std::ptr::null());
            // Compile the Fragment Shader into machine code
            gl::CompileShader(shader);
            // Check for compilation errors
            check_shader_compile_errors(shader, "FRAGMENT");
            shader
        };

        // Create Shader Program Object and get its reference
        let program_id = unsafe {
            let program = gl::CreateProgram();
            // Attach the Vertex and Fragment Shaders to the Shader Program
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            // Link all shaders into a Shader Program
            gl::LinkProgram(program);
            // Check for linking errors
            check_program_link_errors(program);
            program
        };

        // Delete the now useless Vertex and Fragment Shader Objects
        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(Self { ID: program_id })
    }

    // Activates Shader Program
    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.ID);
        }
    }

    // Deletes Shader Program 
    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.ID);
        }
    }
}