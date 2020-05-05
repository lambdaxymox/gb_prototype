#version 330 core

in vec2 ov_tex_coord;
uniform sampler2D f_tex;
out vec4 of_frag_color;


void main() {
    of_frag_color = texture(f_tex, ov_tex_coord);
}
