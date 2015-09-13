#version 150

in vec4 f_color;
in vec3 world_normal;
out vec4 out_color;

uniform float ambient_intensity;
uniform vec3 light_direction;
uniform float directional_intensity;

void main() {
  float cos_directional_angle_incidence = clamp(dot(normalize(world_normal), -light_direction), 0, 1);

  out_color = f_color * ambient_intensity
    + f_color * cos_directional_angle_incidence * directional_intensity;
}
