#version 450

layout(location = 0) in vec2 tex_coord;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 2) uniform sampler2D tex;

layout(set = 0, binding = 1) uniform Data {
    vec4 color;
} uniforms;

void main() {
    f_color = texture(tex, tex_coord).r * uniforms.color;
}
