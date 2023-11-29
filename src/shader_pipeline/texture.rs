use gl::types::*;
use std::ffi::{CString};
use std::ptr;
use image::{DynamicImage, GenericImageView};
use crate::Shader;

pub struct Texture {
    pub id: GLuint,
    pub tex_type: GLenum,
}

impl Texture {
    pub fn new(image_path: &str, tex_type: GLenum, slot: GLenum) -> Result<Self, String> {
        let mut texture = Texture { id: 0, tex_type };

        // Generate texture ID
        unsafe {
            gl::GenTextures(1, &mut texture.id);
            // Activate the texture unit and bind the texture
            gl::ActiveTexture(slot);
            gl::BindTexture(tex_type, texture.id);
        }

        // Set the texture wrapping/filtering options (optional)
        unsafe {
            // Confige the types of algo that are used to resize the image
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // Configure the way the texture repeats (if it does at all)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        // Load and generate the texture
        let image = image::open(image_path).map_err(|e| e.to_string())?;
        let (width, height) = image.dimensions();
        let data = image.into_bytes();

        unsafe {
            // Assign the image to a Texture Object
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
            // Generate MipMaps
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        // Unbind the texture
        texture.unbind();

        Ok(texture)
    }

    pub fn tex_unit(&self, shader: &Shader, uniform: &str, unit: GLuint) {
        unsafe {
            // Get location for the uniform
            let tex_uni = gl::GetUniformLocation(shader.id, CString::new(uniform).unwrap().as_ptr());
            // Activate Shader
            shader.activate();
            // Set the value of the uniform
            gl::Uniform1i(tex_uni, unit as i32);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.tex_type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(self.tex_type, 0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
