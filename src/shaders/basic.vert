#version 150

in vec3 position;
in vec3 normal;

out vec4 f_color;
out vec3 world_normal;

uniform vec3 color;

uniform mat4 proj;
uniform mat4 model;
uniform mat4 camera;

void main() {
  gl_Position = proj * camera * model * vec4(position, 1.0);
  world_normal = vec3(model * vec4(normal, 0.0));

  f_color = vec4(color, 1.0);
}
