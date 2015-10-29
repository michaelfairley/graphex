extern crate sdl2;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate glium_text;
#[macro_use]
extern crate lazy_static;

use cgmath::EuclideanVector;
use cgmath::Vector;
use cgmath::Rotation;
use cgmath::Rotation3;
use cgmath::Matrix;
use cgmath::FixedArray;

use std::f32::consts::PI;

pub mod shapes;
pub mod fps;
pub mod ring;
pub mod mirror;

const CAMERA_ROTATE_SPEED: f32 = 1.0/100.0;
const MOVE_SPEED: f32 = 0.1;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const FONT_SIZE: u32 = 24;

fn main() {
  use glium_sdl2::DisplayBuild;
  use glium::Surface;

  let sdl_context = sdl2::init().unwrap();

  let video = sdl_context.video().unwrap();

  let gl_attr = video.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 2);
  gl_attr.set_context_flags().debug().set();
  gl_attr.set_stencil_size(8);

  let window = video
    .window("Graphex", WIDTH, HEIGHT)
    .resizable()
    .build_glium()
    .unwrap();

  let text_system = glium_text::TextSystem::new(&window);
  let font = glium_text::FontTexture::new(&window, std::fs::File::open(&std::path::Path::new("assets/OpenSans-Regular.ttf")).unwrap(), FONT_SIZE).unwrap();

  let proj = cgmath::perspective(cgmath::deg(60 as f32), 1024.0/768.0, 0.1, 45.0);

  let light_direction = cgmath::vec3::<f32>(1.0, -2.0, 1.0).normalize();

  let basic_program = glium::Program::from_source(&window, include_str!("shaders/basic.vert"), include_str!("shaders/basic.frag"), None).unwrap();
  let flat_program = glium::Program::from_source(&window, include_str!("shaders/flat.vert"), include_str!("shaders/flat.frag"), None).unwrap();

  let cube_vertex_buffer = glium::VertexBuffer::new(&window, &shapes::CUBE).unwrap();
  let sphere_vertex_buffer = glium::VertexBuffer::new(&window, &shapes::sphere(3)).unwrap();
  let plane_vertex_buffer = glium::VertexBuffer::new(&window, &shapes::PLANE).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let basic_params = glium::DrawParameters {
    depth_test: glium::DepthTest::IfLessOrEqual,
    depth_range: (0.0, 1.0),
    depth_write: true,
    backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
    .. Default::default()
  };

  let mirror_params = glium::DrawParameters {
    stencil_test_clockwise: glium::StencilTest::AlwaysPass,
    stencil_depth_pass_operation_clockwise: glium::StencilOperation::Replace,
    stencil_reference_value_clockwise: 1,
    depth_write: false,
    .. basic_params.clone()
  };

  let reflected_params = glium::DrawParameters {
    stencil_test_counter_clockwise: glium::StencilTest::IfEqual { mask: 0xFF },
    stencil_reference_value_counter_clockwise: 1,
    depth_test: glium::DepthTest::IfLessOrEqual,
    depth_range: (0.0, 1.0),
    depth_write: true,
    backface_culling: glium::BackfaceCullingMode::CullClockWise,
    .. basic_params.clone()
  };

  let mirrors = vec![
    mirror::Mirror { buffer: &plane_vertex_buffer, color: [0.9, 0.9, 0.9], position: cgmath::vec3(3.0, 1.0, -9.4), rotation: cgmath::Matrix3::identity() },
    mirror::Mirror { buffer: &plane_vertex_buffer, color: [0.9, 0.9, 0.9], position: cgmath::vec3(-9.4, 1.0, 3.0), rotation: cgmath::Matrix3::from(cgmath::Basis3::from_angle_y(cgmath::rad(PI/2.0))) },
    ];

  let shapes = vec![
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [1.0, 1.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(3.0, 1.0, -5.0)), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [1.0, 0.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(-3.0, 1.0, -5.0)), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &sphere_vertex_buffer, color: [1.0, 1.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 1.0, -5.0)), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &sphere_vertex_buffer, color: [1.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(-6.0, 1.0, -5.0)), shading: shapes::Shading::Flat },
    // Walls
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 1.0, -10.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(21.0, 5.0, 1.0)).into()), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 1.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 1.0, 10.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(21.0, 5.0, 1.0)).into()), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [1.0, 0.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(10.0, 1.0, 0.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(1.0, 5.0, 21.0)).into()), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 0.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(-10.0, 1.0, 0.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(1.0, 5.0, 21.0)).into()), shading: shapes::Shading::Smooth },
    ];

  let mut running = true;

  let mut camera_position = cgmath::vec3::<f32>(0.0, 0.0, 0.0);
  let mut look_up = 0.0f32;
  let mut look_right = 0.0f32;

  let mut fps = fps::new(sdl_context.timer().unwrap());

  let mut events = sdl_context.event_pump().unwrap();

  let mouse = sdl_context.mouse();
  mouse.set_relative_mouse_mode(true);

  while running {
    fps.tick();
    for event in events.poll_iter() {
      use sdl2::event::Event;
      use sdl2::keyboard::Keycode;

      match event {
        Event::Quit {..}
        | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
          running = false,
        Event::MouseMotion { xrel: dx, yrel: dy, .. } => {
          look_right += dx as f32 * CAMERA_ROTATE_SPEED;
          look_up += dy as f32 * CAMERA_ROTATE_SPEED;
          look_up = look_up.max(-PI/2.0).min(PI/2.0);
        },
        _ => {}
      }
    }

    let camera_rotation = cgmath::Basis3::from_angle_x(cgmath::rad(look_up)).concat(&cgmath::Basis3::from_angle_y(cgmath::rad(look_right)));

    {
      use sdl2::keyboard::Scancode;

      let keystates = events.keyboard_state();
      let mut camera_change = cgmath::vec3::<f32>(0.0, 0.0, 0.0);
      if keystates.is_scancode_pressed(Scancode::D) {
        camera_change.add_self_v(&cgmath::vec3(MOVE_SPEED, 0.0, 0.0));
      }
      if keystates.is_scancode_pressed(Scancode::A) {
        camera_change.add_self_v(&cgmath::vec3(-MOVE_SPEED, 0.0, 0.0));
      }
      if keystates.is_scancode_pressed(Scancode::W) {
        camera_change.add_self_v(&cgmath::vec3(0.0, 0.0, -MOVE_SPEED));
      }
      if keystates.is_scancode_pressed(Scancode::S) {
        camera_change.add_self_v(&cgmath::vec3(0.0, 0.0, MOVE_SPEED));
      }
      camera_position.add_self_v(&camera_rotation.invert().rotate_vector(&camera_change));
    }

    let camera = cgmath::Matrix4::from(cgmath::Matrix3::from(camera_rotation)).mul_m(&cgmath::Matrix4::from_translation(&camera_position.mul_s(-1.0)));

    let mut target = window.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    for shape in &shapes {

      let basic_uniforms = uniform! {
        color: shape.color,
        model: shape.matrix,
        camera: camera,
        proj: proj,
        ambient_intensity: 0.5f32,
        directional_intensity: 0.5f32,
        light_direction: light_direction,
      };

      let program = match shape.shading {
        shapes::Shading::Flat => &flat_program,
        shapes::Shading::Smooth => &basic_program,
      };

      target.draw(shape.buffer,
                  &indices,
                  program,
                  &basic_uniforms,
                  &basic_params).unwrap();
    }

    for mirror in &mirrors {

      let basic_uniforms = uniform! {
        color: mirror.color,
        model: cgmath::Matrix4::from_translation(&mirror.position).mul_m(&mirror.rotation.into()),
        camera: camera,
        proj: proj,
        ambient_intensity: 0.5f32,
        directional_intensity: 0.5f32,
        light_direction: light_direction,
      };

      target.clear_stencil(0);

      target.draw(mirror.buffer,
                  &indices,
                  &basic_program,
                  &basic_uniforms,
                  &mirror_params).unwrap();


      let mirror_normal = mirror.rotation.mul_v(&cgmath::vec3(0.0, 0.0, 1.0));

      let camera_to_mirror = mirror.position.sub_v(&camera_position);

      let camera_reflection = mirror_normal.mul_s(camera_to_mirror.dot(&mirror_normal) * 2.0);

      let reflected_camera_position = camera_position.add_v(&camera_reflection);

      let reflected_camera_rotation = cgmath::Basis3::look_at(&reflected_camera_position.sub_v(&mirror.position), &cgmath::vec3(0.0, 1.0, 0.0));

      let reflected_camera = cgmath::Matrix4::from(cgmath::Matrix3::from(reflected_camera_rotation)).mul_m(&cgmath::Matrix4::from_translation(&reflected_camera_position.mul_s(-1.0)));

      let mirror_proj = mirror_projection(&proj, &reflected_camera, &mirror);

      for shape in &shapes {

        let uniforms = uniform! {
          color: shape.color,
          model: shape.matrix,
          camera: reflected_camera,
          proj: mirror_proj,
          ambient_intensity: 0.5f32,
          directional_intensity: 0.5f32,
          light_direction: light_direction,
        };

        let program = match shape.shading {
          shapes::Shading::Flat => &flat_program,
          shapes::Shading::Smooth => &basic_program,
        };

        target.draw(shape.buffer,
                    &indices,
                    program,
                    &uniforms,
                    &reflected_params).unwrap();
      }
    }

    let fps_text = glium_text::TextDisplay::new(&text_system, &font, &format!("{} fps", fps.average()));

    let fps_text_matrix = cgmath::Matrix4::from_translation(&cgmath::vec3(-1.0, -1.0, 0.0)).mul_m(&cgmath::Matrix4::from(cgmath::Matrix3::from(cgmath::Matrix2::from_value(FONT_SIZE as f32 / HEIGHT as f32))));

    glium_text::draw(&fps_text, &text_system, &mut target, fps_text_matrix.into_fixed(), (1.0, 1.0, 1.0, 1.0));

    target.finish().unwrap();
  }
}

// http://www.terathon.com/code/oblique.html
fn mirror_projection(proj: &cgmath::Matrix4<f32>,
                     camera: &cgmath::Matrix4<f32>,
                     mirror: &mirror::Mirror)-> cgmath::Matrix4<f32> {

  let c_mirror_position = camera.mul_v(&mirror.position.extend(1.0)).truncate();

  let mirror_normal = camera.mul_v(&mirror.rotation.mul_v(&cgmath::vec3(0.0, 0.0, 1.0)).extend(0.0)).truncate();

  let mirror_plane = mirror_normal.extend(mirror_normal.mul_s(-1.0).dot(&c_mirror_position));

  let q = cgmath::vec4((mirror_plane.x.signum() + proj[2][0]) / proj[0][0],
                       (mirror_plane.y.signum() + proj[2][1]) / proj[1][1],
                       -1.0,
                       (1.0 + proj[2][2]) / proj[3][2]);

  let c = mirror_plane.mul_s(2.0 / mirror_plane.dot(&q));

  let mut result = proj.clone();

  result[0][2] = c.x;
  result[1][2] = c.y;
  result[2][2] = c.z + 1.0;
  result[3][2] = c.w;

  return cgmath::Matrix4::new(-1.0, 0.0, 0.0, 0.0,
                               0.0, 1.0, 0.0, 0.0,
                               0.0, 0.0, 1.0, 0.0,
                               0.0, 0.0, 0.0, 1.0).mul_m(&result);
}
