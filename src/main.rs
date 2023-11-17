#![allow(unused_imports, dead_code)]

extern crate gl;
extern crate gl_generator;
extern crate glfw;
extern crate image;

use std::os::raw::c_void;
use std::ffi::{CString};
use std::ptr;
use std::fs;
use gl::types::*;
use glfw::{Action, Context, Key};
use glfw::fail_on_errors;
use crate::image::GenericImageView;

// mod shader_pipeline;

fn main() {
    // Load the image
    let img = image::open("C:\\Users\\pasca\\My Game\\shaders\\assets\\sample_texture.jpg").unwrap();
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
        .create_window(800, 600, "Red Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Create the texture
    let texture_id = load_texture(&img).unwrap();
    
    // Read the shader code from a file
    // let shader_source = fs::read_to_string("C:\\Users\\pasca\\My Game\\shaders\\assets\\shadercode\\simple_sobel_shader.glsl").expect("Failed to read shader file");
    //let vertex_shader_source = fs::read_to_string("C:\\Users\\pasca\\My Game\\shaders\\assets\\shadercode\\vertex_test.glsl").expect("Failed to read shader file");
    //let fragment_shader_source = fs::read_to_string("C:\\Users\\pasca\\My Game\\shaders\\assets\\shadercode\\fragment_test.glsl").expect("Failed to read shader file");

    // Compile the shader
    let vertex_shader: u32 = compile_shader(&vertex_shader_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fragment_shader_source, gl::FRAGMENT_SHADER);

    // Link the shader program
    let program = link_program(vertex_shader, fragment_shader);

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
        //window.swap_buffers();

        unsafe {
            // Specify the color of the background
            gl::ClearColor(0.07f32, 0.13f32, 0.17f32, 1.0f32);
            // Clean the back buffer and assign the new color
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Tell OpenGL which shader program to use 
            gl::UseProgram(program);
            // Bind the vao so OpenGL knows to use it
            gl::BindVertexArray(vao);
            // Draw the triangles using GL_TRIANGLES primitive
            gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT, ptr::null());
            window.swap_buffers();
        }

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
    println!("{}", texture_id);
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

// Function to link a shader program
fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Done using the shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

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

fn initialize_opengl() -> (GLuint, GLuint, GLuint) {
    //println!("Inside GL initiation.");

    let vertices: [f32; 18] = [
        -0.5f32, (-0.5f32 * 3.0f32.sqrt()) / 3f32, 0.0f32, // Lower left corner
        0.5f32, (-0.5f32 * 3.0f32.sqrt()) / 3f32, 0.0f32, // Lower right corner
        0.0f32 / 2f32, (0.5f32 * 3.0f32.sqrt()) * 2f32 / 3f32, 0.0f32, // Upper corner
        -0.5f32 / 2f32, (0.5f32 * 3.0f32.sqrt()) / 6f32, 0.0f32, // Inner left
        0.5f32 / 2f32, (0.5f32 * 3.0f32.sqrt()) / 6f32, 0.0f32, // Inner right
        0.0f32, (0.5f32 * 3.0f32.sqrt()), 0.0f32, // Inner down
    ];

    // Vertex sharing
    let indices: [u32; 9] = [
        0, 3, 5, // Lower left triangle
        3, 2, 4, // Lower right triangle 
        5, 4, 1, // Upper triangle
    ]; 

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        // Generate vao and vbo with 1 object each
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // Make the vao the current vertex array object by binding it 
        gl::BindVertexArray(vao);

        // Bind the vbo
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Introduce vertices to vbo
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);

        // Configure the vertex attribute so that OpenGL knows how to read the vbo
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, ptr::null());
        // Enable the vertex attribute so that OpenGL knows how to use it
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    (vao, vbo, ebo)
}