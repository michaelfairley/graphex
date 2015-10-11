#version 150

in vec3 position;
in vec3 normal;

flat out vec4 f_color;

uniform vec3 color;

uniform mat4 proj;
uniform mat4 model;
uniform mat4 camera;

uniform float ambient_intensity;
uniform vec3 light_direction;
uniform float directional_intensity;

void main() {
  gl_Position = proj * camera * model * vec4(position, 1.0);
  vec3 world_normal = vec3(model * vec4(normal, 0.0));

  float cos_directional_angle_incidence = clamp(dot(normalize(world_normal), -light_direction), 0, 1);

  f_color = vec4(color * ambient_intensity
                 + color * cos_directional_angle_incidence * directional_intensity, 1.0);

}
