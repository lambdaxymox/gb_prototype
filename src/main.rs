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
mod mesh;

use gl_backend as glh;
use mesh::Mesh;

use cgmath::{
    Array, 
    Matrix4,
    One
};
use glfw::{
    Action, 
    Context, 
    Key
};
use gl::types::{
    GLfloat, 
    GLint, 
    GLuint, 
    GLvoid, 
    GLsizeiptr,
    GLsizei,
    GLenum,
};
use log::{info};
use teximage2d::TexImage2D;

use std::io;
use std::ptr;

// OpenGL extension constants.
const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;


/// Load texture image into the GPU.
fn send_to_gpu_texture(image: &TexImage2D, wrapping_mode: GLuint) -> Result<GLuint, String> {
    let mut tex = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
    }
    debug_assert!(tex > 0);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width as i32, image.height as i32, 0,
            gl::RGBA, gl::UNSIGNED_BYTE,
            image.as_ptr() as *const GLvoid
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping_mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping_mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
    }

    let mut max_aniso = 0.0;
    unsafe {
        gl::GetFloatv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_aniso);
        // Set the maximum!
        gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, max_aniso);
    }

    Ok(tex)
}

#[derive(Copy, Clone)]
struct ShaderSource {
    vert_name: &'static str,
    vert_source: &'static str,
    frag_name: &'static str,
    frag_source: &'static str,
}

fn send_to_gpu_shaders(game: &mut glh::GLState, source: ShaderSource) -> GLuint {
    let mut vert_reader = io::Cursor::new(source.vert_source);
    let mut frag_reader = io::Cursor::new(source.frag_source);
    let sp = glh::create_program_from_reader(
        game,
        &mut vert_reader, source.vert_name,
        &mut frag_reader, source.frag_name
    ).unwrap();
    debug_assert!(sp > 0);

    sp
}

fn create_mesh_triangle(h: f32) -> Mesh {
    let a = f32::sqrt(4_f32 * h * h / 3_f32);
    let half_a = a / 2_f32;
    let half_h = h / 2_f32;


    let points = [
        [-half_a, -half_h, 0_f32], [half_a, -half_h, 0_f32], [0_f32, half_h, 0_f32],
    ];
    let tex_coords = [
        [0_f32, 0_f32], [1_f32, 0_f32], [0.5, 1_f32],
    ];

    Mesh::new(&points, &tex_coords)
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

#[derive(Copy, Clone)]
struct Handle {
    vao: GLuint,
    v_pos_vbo: GLuint,
    v_tex_vbo: GLuint,
    v_pos_loc: GLuint,
    v_tex_loc: GLuint,
}

fn create_buffers_triangle(sp: GLuint) -> Handle {
    let v_pos_loc = unsafe {
        gl::GetAttribLocation(sp, glh::gl_str("v_pos").as_ptr())
    };
    debug_assert!(v_pos_loc > -1);
    let v_pos_loc = v_pos_loc as GLuint;
    
    let v_tex_loc = unsafe {
        gl::GetAttribLocation(sp, glh::gl_str("v_tex").as_ptr())
    };
    debug_assert!(v_tex_loc > -1);
    let v_tex_loc = v_tex_loc as GLuint;

    let mut v_pos_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut v_pos_vbo);
    }
    debug_assert!(v_pos_vbo > 0);

    let mut v_tex_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut v_tex_vbo);
    }
    debug_assert!(v_tex_vbo > 0);

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    debug_assert!(vao > 0);
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_pos_vbo);
        gl::VertexAttribPointer(v_pos_loc, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, v_tex_vbo);
        gl::VertexAttribPointer(v_tex_loc, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_loc);
        gl::EnableVertexAttribArray(v_tex_loc);
    }

    Handle {
        vao: vao,
        v_pos_vbo: v_pos_vbo,
        v_tex_vbo: v_tex_vbo,
        v_pos_loc: v_pos_loc,
        v_tex_loc: v_tex_loc,
    }
}

fn send_to_gpu_geometry_triangle(handle: Handle, mesh: &Mesh) {
    unsafe {
        // Load position data.
        gl::BindBuffer(gl::ARRAY_BUFFER, handle.v_pos_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mesh.points.len_bytes() as GLsizeiptr,
            mesh.points.as_ptr() as *const GLvoid, gl::STATIC_DRAW
        );
        // Load the texture coordinates.
        gl::BindBuffer(gl::ARRAY_BUFFER, handle.v_tex_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mesh.tex_coords.len_bytes() as GLsizeiptr,
            mesh.tex_coords.as_ptr() as *const GLvoid, gl::STATIC_DRAW
        );

        // Enable the arrays for use by the shader.
        gl::BindVertexArray(handle.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, handle.v_pos_vbo);
        gl::VertexAttribPointer(handle.v_pos_loc, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, handle.v_tex_vbo);
        gl::VertexAttribPointer(handle.v_tex_loc, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(handle.v_pos_loc);
        gl::EnableVertexAttribArray(handle.v_tex_loc);
    }
}

fn send_to_gpu_uniforms_triangle(sp: GLuint, trans_mat: Matrix4, scale_mat: Matrix4) {
    let scale_mat_loc = unsafe {
        gl::GetUniformLocation(sp, glh::gl_str("v_scale_mat").as_ptr())
    };
    debug_assert!(scale_mat_loc > -1);
    let trans_mat_loc = unsafe {
        gl::GetUniformLocation(sp, glh::gl_str("v_trans_mat").as_ptr())
    };
    debug_assert!(trans_mat_loc > -1);
    unsafe {
        gl::UseProgram(sp);
        gl::UniformMatrix4fv(scale_mat_loc, 1, gl::FALSE, scale_mat.as_ptr());
        gl::UniformMatrix4fv(trans_mat_loc, 1, gl::FALSE, trans_mat.as_ptr());
    }
}


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

fn main() {
    init_logger("gb_prototype.log");
    info!("BEGIN LOG");
    info!("build version: ??? ?? ???? ??:??:??");

    // Load the components of the scene.
    let shaders = create_shaders_triangle();
    let mesh = create_mesh_triangle(1_f32);
    let image = create_textures_triangle();
    let mut gl = init_gl(640, 480);

    // Set them up on the GPU.
    let sp = send_to_gpu_shaders(&mut gl, shaders);
    if !gl_backend::validate_shader_program(sp) {
        info!("Got invalid shader program.");
        info!("{}", gl_backend::program_info_log(sp));
        info!("{}", gl_backend::shader_info_log(sp));
    }

    let mut num_active_attribs = 0;
    let mut num_active_uniforms = 0;
    unsafe {
        gl::GetProgramiv(sp, gl::ACTIVE_ATTRIBUTES, &mut num_active_attribs);
        gl::GetProgramiv(sp, gl::ACTIVE_UNIFORMS, &mut num_active_uniforms);
    }
    println!("num_active_attribsd = {}", num_active_attribs);
    println!("num_active_uniforms = {}", num_active_uniforms);
    let mut max_attrib_name_length = 0;
    unsafe {
        gl::GetProgramiv(sp, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_attrib_name_length);
    }
    let mut name_data: Vec<i8> = vec![0; max_attrib_name_length as usize];
    for attrib in 0..num_active_attribs {
        let mut array_size: GLint = 0;
        let mut typ: GLenum = 0;
        let mut actual_length: GLsizei = 0;
        unsafe {
            gl::GetActiveAttrib(sp, attrib as u32, name_data.len() as i32, &mut actual_length, &mut array_size, &mut typ, &mut name_data[0]);
        }
        let name_data_ptr = unsafe { std::mem::transmute::<&Vec<i8>, &Vec<u8>>(&name_data) };
        println!("{}", std::str::from_utf8(&name_data_ptr[0..actual_length as usize]).unwrap());
    }

    let mut name_data: Vec<i8> = vec![0; 256];
    for uniform in 0..num_active_uniforms {
        let mut array_size: GLint = 0;
        let mut typ: GLenum = 0;
        let mut actual_length: GLsizei = 0;
        unsafe {
            gl::GetActiveUniform(sp, uniform as u32, name_data.len() as i32, &mut actual_length, &mut array_size, &mut typ, &mut name_data[0]);
        }
        let name_data_ptr = unsafe { std::mem::transmute::<&Vec<i8>, &Vec<u8>>(&name_data) };
        println!("{}", std::str::from_utf8(&name_data_ptr[0..actual_length as usize]).unwrap());
    }

    let handle = create_buffers_triangle(sp);
    send_to_gpu_geometry_triangle(handle, &mesh);
    let tex = send_to_gpu_texture(&image, gl::CLAMP_TO_EDGE).unwrap();
    let trans_mat = Matrix4::one();
    let scale_mat = Matrix4::one();
    send_to_gpu_uniforms_triangle(sp, trans_mat, scale_mat);

    while !gl.window.should_close() {
        gl.glfw.poll_events();
        match gl.window.get_key(Key::Escape) {
            Action::Press | Action::Repeat => {
                gl.window.set_should_close(true);
            }
            _ => {}
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Viewport(0, 0, 640, 480);
        }

        // Update the GPU.
        send_to_gpu_uniforms_triangle(sp, trans_mat, scale_mat);

        // Render the results.
        unsafe {
            gl::UseProgram(sp);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::BindVertexArray(handle.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        gl.window.swap_buffers();
    }
    info!("END LOG");
}
