/*
 *  GB-Backend is a renderer prototype demo.
 *  Copyright (C) 2018,2019,2020  Christopher Blanchard
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
extern crate glfw;
extern crate cgmath;
extern crate toml;
extern crate log;
extern crate rand;
extern crate file_logger;
extern crate teximage2d;


mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod gl_backend;

use gl_backend as glh;
use glfw::{Action, Context, Key};
use gl::types::{GLfloat, GLint, GLuint, GLvoid, GLsizeiptr};
use log::{info};
use teximage2d::TexImage2D;


const CLEAR_COLOR: [f32; 4] = [0.2_f32, 0.2_f32, 0.2_f32, 1.0_f32];

// OpenGL extension constants.
const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;


/// Initialize the logger.
fn init_logger(log_file: &str) {
    file_logger::init(log_file).expect("Failed to initialize logger.");
}

/// Create and OpenGL context.
fn init_gl(width: u32, height: u32) -> glh::GLState {
    let gl_state = match glh::start_gl(width, height) {
        Ok(val) => val,
        Err(e) => {
            panic!("Failed to Initialize OpenGL context. Got error: {}", e);
        }
    };

    gl_state
}

struct Mesh {
    points: [[f32; 2]; 3],
    tex_coords: [[f32; 2]; 3],
}

fn create_meshj_triangle(h: f32) -> Mesh {
    let a = f32::sqrt(4_f32 * h * h / 3_f32);
    let half_a = a / 2_f32;
    let half_h = h / 2_f32;


    let points = [
        [-half_a, -half_h], [half_a, -half_h], [0_f32, half_h],
    ];
    let tex_coords = [
        [0_f32, 0_f32], [1_f32, 0_f32], [0.5, 1_f32],
    ];

    Mesh {
        points: points,
        tex_coords: tex_coords,
    }
}

#[derive(Copy, Clone)]
struct ShaderSource {
    vert_name: &'static str,
    vert_source: &'static str,
    frag_name: &'static str,
    frag_source: &'static str,
}

fn create_shaders_triangle() -> ShaderSource {
    let vert_source = include_str!("../shaders/triangle.vert.glsl");
    let frag_source = include_str!("../shaders/triangle.frag.glsl");

    ShaderSource { 
        vert_name: "triangle.vert.glsl",
        vert_source: vert_source,
        frag_name: "triangle.frag.glsl",
        frag_source: frag_source,
    }
}

fn create_textures_triangle() -> TexImage2D {
    let asset = include_bytes!("../assets/marble.png");
    teximage2d::load_from_memory(asset).unwrap().image
}

fn main() {
    init_logger("gb_prototype.log");
    info!("BEGIN LOG");
    info!("build version: ??? ?? ???? ??:??:??");
    let mut gl = init_gl(640, 480);
    while !gl.window.should_close() {
        gl.glfw.poll_events();
        match gl.window.get_key(Key::Escape) {
            Action::Press | Action::Repeat => {
                gl.window.set_should_close(true);
            }
            _ => {}
        }

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &CLEAR_COLOR[0] as *const GLfloat);
        }

        gl.window.swap_buffers();
    }
    info!("END LOG");
}
