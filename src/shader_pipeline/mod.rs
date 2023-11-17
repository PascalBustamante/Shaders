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


mod shader_pipeline {

    pub fn load_texture(image: &image::DynamicImage) -> Result<GLuint, Box<dyn std::error::Error>>{
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

        // Set up a quad for rendering
        let vertices: [f32; 15] = [
            -0.5f, -0.5f * gl::FLOAT(sqrt(3)) / 3, 0.0f, // Lower left corner
            0.5f, -0.5f * gl::FLOAT(sqrt(3)) / 3, 0.0f, // Lower right corner
            0.0f / 2, 0.5 * gl::FLOAT(sqrt(3)) * 2 / 3, 0.0f, // Upper corner
            -0.5f / 2, 0.5f * gl::FLOAT(sqrt(3)) / 6, 0.0f, // Inner left
            0.5f / 2, 0.5f * gl::FLOAT(sqrt(3)) / 6, 0.0f, // Inner right
            0.0f, 0.5f * gl::FLOAT(sqrt(3)), 0.0f, // Inner down
        ];

        // Vertex sharing
        let indices: [u32; 6] = [
            0, 3, 5, // Lower left triangle
            3, 2, 4, // Lower right triangle 
            5, 4, 1, // Upper triangle
        ];

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error: {}", error);
            }

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error: {}", error);
            }

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);

            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error: {}", error);
            }

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error: {}", error);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        (vao, vbo, ebo)
    }
}
