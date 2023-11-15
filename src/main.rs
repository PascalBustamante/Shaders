extern crate gl;
extern crate gl_generator;
extern crate glfw;
extern crate image;

use std::os::raw::c_void;
use std::ffi::{CString, CStr};
use std::ptr;
use std::fs;
use gl::types::*;
use glfw::{Action, Context, Key};
use glfw::fail_on_errors;
use crate::image::GenericImageView;

fn main() {
    // Load the image
    let img = image::open("C:\\Users\\pasca\\My Game\\bevy_playground\\assets\\Provinces_2600_100_3600_1000.png").unwrap();
    //let width = img.width();
    //let height = img.height(); //these 3 might not be needed 
    //let raw_pixels = img.into_bytes();

    // Create a GL context using glfw
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    println!("GLFW initialized successfully.");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello, I'm a window!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Create the texture
    let texture_id = load_texture(&img).unwrap();
    
    // Read the shader code from a file
    let shader_source = fs::read_to_string("C:\\Users\\pasca\\My Game\\bevy_playground\\assets\\shaders\\simple_sobel_shader.glsl").expect("Failed to read shader file");

    // Compile the shader
    let fragment_shader = compile_shader(&shader_source, gl::FRAGMENT_SHADER);

    // Link the shader program
    let program = link_program(fragment_shader);

    // Initialize OpenGL
    let (vao, vbo, ebo) = initialize_opengl();

    // Checking errs
    unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("OpenGL error: {}", error);
        }
    }

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }

        // Render the quad with the shader
        render_quad(program, texture_id, vao);
    }

    // Clean up OpenGL resources before exiting
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
        gl::DeleteTextures(1, &texture_id);
        gl::DeleteProgram(program);
    }
}


// Funtion to load a 2d image as a texture

fn load_texture(image: &image::DynamicImage) -> Result<GLuint, Box<dyn std::error::Error>>{
    // Get the image dimensions
    let (width, height) = image.dimensions();

    // Get the image data as a flat Vec<u8>
    let rgba_data: Vec<u8> = image.to_rgba8().into_raw();

    // Generate texture ID
    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Upload image data to the texture
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            rgba_data.as_ptr() as *const c_void,
        );
    }

    Ok(texture_id)
}

// Function to compile a GLSL shader
fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).expect("CString::new failed");
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check compilation status
        let mut success: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            let mut len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize - 1));
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            let error_message = String::from_utf8_lossy(&buffer);
            panic!("Shader compilation failed: {}", error_message);
        }

        shader
    }
}

// Function to link a shader program
fn link_program(fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Check linking status
        let mut success: GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
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

        program
    }
}

// Function to render a quad with the given shader and texture
fn render_quad(program: GLuint, texture_id: GLuint, vao: GLuint) {
    unsafe {
        // Use the shader program
        gl::UseProgram(program);

        // Activate a texture unit
        gl::ActiveTexture(gl::TEXTURE0);

        // Bind the texture to the active texture unit
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        // Get the location of the 'inputTexture' uniform in the shader
        let texture_uniform_location = gl::GetUniformLocation(program, "inputTexture\0".as_ptr() as *const i8);

        // Set the texture unit index as the value for the 'inputTexture' uniform
        gl::Uniform1i(texture_uniform_location, 0);

        // Render the quad
        gl::BindVertexArray(vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}

fn initialize_opengl() -> (GLuint, GLuint, GLuint) {
    // Set up a quad for rendering
    let vertices: [f32; 12] = [
        -1.0, -1.0, 0.0,
         1.0, -1.0, 0.0,
         1.0,  1.0, 0.0,
        -1.0,  1.0, 0.0,
    ];

    // A rectangle as 2 triangles
    let indices: [u32; 6] = [0, 1, 2, 0, 2, 3]; 

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    (vao, vbo, ebo)
}