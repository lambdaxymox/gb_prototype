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


mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod gl_backend;

use gl_backend as glh;
use glfw::{Action, Context, Key};
use gl::types::{GLfloat, GLint, GLuint, GLvoid, GLsizeiptr};
use log::{info};


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
    let mut gl = init_gl(640, 480);
    while !gl.window.should_close() {
        gl.glfw.poll_events();
        match gl.window.get_key(Key::Escape) {
            Action::Press | Action::Repeat => {
                gl.window.set_should_close(true);
            }
            _ => {}
        }

        gl.window.swap_buffers();
    }
    info!("END LOG");
}
