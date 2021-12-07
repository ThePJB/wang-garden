layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_colour;

uniform mat4 projection;

out vec4 vert_colour;

void main() {
    vert_colour = in_colour;
    gl_Position = projection * vec4(in_pos, 1.0);
}