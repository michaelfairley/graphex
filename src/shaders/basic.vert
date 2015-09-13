#version 150

in vec3 position;
in vec3 normal;

out vec3 f_color;

uniform vec3 color;

uniform mat4 proj;
uniform mat4 model;
uniform mat4 camera;

void main() {
  gl_Position = proj * camera * model * vec4(position, 1.0);

  f_color = color;
}
