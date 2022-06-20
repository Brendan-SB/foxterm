#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 tex_coords;

layout(set = 0, binding = 0) uniform Data {
    	mat4 proj;
	mat4 transform;
} uniforms;

void main() {
	tex_coords = uv;
	gl_Position = uniforms.proj * (uniforms.transform * vec4(position, 1.0));
}
