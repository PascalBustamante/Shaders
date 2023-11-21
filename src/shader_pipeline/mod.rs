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

pub mod VAO;
pub mod VBO;
pub mod EBO;
pub mod Shader;