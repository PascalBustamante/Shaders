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

mod shader_pipeline;

use shader_pipeline::VAO::VAO;
use shader_pipeline::VBO::VBO;
use shader_pipeline::EBO::EBO;
use shader_pipeline::Shader::Shader;

fn main() {
    // Vertices coordinates
    let mut vertices: [GLfloat; 18] = [
        -0.5, -0.5 * f32::sqrt(3.0) / 3.0, 0.0, // Lower left corner
        0.5, -0.5 * f32::sqrt(3.0) / 3.0, 0.0, // Lower right corner
        0.0, 0.5 * f32::sqrt(3.0) * 2.0 / 3.0, 0.0, // Upper corner
        -0.5 / 2.0, 0.5 * f32::sqrt(3.0) / 6.0, 0.0, // Inner left
        0.5 / 2.0, 0.5 * f32::sqrt(3.0) / 6.0, 0.0, // Inner right
        0.0, -0.5 * f32::sqrt(3.0) / 3.0, 0.0 // Inner down
    ];

    // Indices for vertices order
    let mut indices: [GLuint; 9] = [
        0, 3, 5, // Lower left triangle
        3, 2, 4, // Lower right triangle
        5, 4, 1 // Upper triangle
    ];

    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    // Specify OpenGL version and profile
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a GLFW window
    let (mut window, events) = glfw
        .create_window(800, 800, "OpenGL Playground", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // Make window's context current
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Specify the viewport
    unsafe {
        gl::Viewport(0, 0, 800, 800);
    }

    // Shader source code
    let vertex_file = "C:\\Users\\pasca\\My Game\\shaders\\assets\\shadercode\\vertex_test.glsl";
    let fragment_file = "C:\\Users\\pasca\\My Game\\shaders\\assets\\shadercode\\fragment_test.glsl";

    // Generate Shader object using shaders default.vert and default.frag
    // Check if Shader creation was successful
    match Shader::new(vertex_file, fragment_file) {
        Ok(shader_program) => {
            // Shader creation successful

            // Generate Vertex Array Object and bind it
            let mut vao = VAO::new();
            vao.bind();

            // Generate Vertex Buffer Object and link it to verticies
            let mut vbo = VBO::new(&vertices);

            // Generate Element Buffer Object and link it to indices
            let mut ebo = EBO::new(&indices);

            // Link VBO to VAO
            vao.link_vbo(&vbo, 0);

            // Unbind all to prevent accidental modifications
            vao.unbind();
            vbo.unbind();
            ebo.unbind();

            // Loop until the user closes the window
            
            while !window.should_close() {
                unsafe {
                    // Specify the color of the background
                    gl::ClearColor(0.07f32, 0.13f32, 0.17f32, 1.0f32);
                    // Clean the back buffer and assign the new color
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    // Tell OpenGL which shader program to use 
                    shader_program.activate();
                    // Bind the vao so OpenGL knows to use it
                    vao.bind();
                    // Draw the triangles using GL_TRIANGLES primitive
                    gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT, ptr::null());
                    // Swap front and back buffers
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
                // render_quad(program, texture_id, vao);
            }
        
            // Delete all the objects 
            vao.delete();
            vbo.delete();
            ebo.delete();
            shader_program.delete();
        }

        Err(error) => {
            // Shader creation failed, handle or log the error
            eprintln!("Error creating shader program: {}", error);

            // Optionally, you might want to exit the program or take appropriate action
        }
    }
}

