#version 330 core

layout (location = 0) in vec2 v_pos;
layout (location = 1) in vec2 v_tex;
uniform mat4 v_scale_mat;
uniform mat4 v_trans_mat;

out vec2 ov_tex_coord;


void main() {
    ov_tex_coord = v_tex;
    gl_Position = v_trans_mat * v_scale_mat * vec4(v_pos, 0.0, 1.0);
}
