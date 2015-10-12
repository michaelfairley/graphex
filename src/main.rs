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

  let window = video
    .window("Graphex", WIDTH, HEIGHT)
    .resizable()
    .build_glium()
    .unwrap();

  let text_system = glium_text::TextSystem::new(&window);
  let font = glium_text::FontTexture::new(&window, std::fs::File::open(&std::path::Path::new("assets/OpenSans-Regular.ttf")).unwrap(), FONT_SIZE).unwrap();

  let proj = cgmath::perspective(cgmath::deg(60 as f32), 1024.0/768.0, 1.0, 45.0);

  let light_direction = cgmath::vec3::<f32>(1.0, -2.0, 1.0).normalize();

  let basic_program = glium::Program::from_source(&window, include_str!("shaders/basic.vert"), include_str!("shaders/basic.frag"), None).unwrap();
  let flat_program = glium::Program::from_source(&window, include_str!("shaders/flat.vert"), include_str!("shaders/flat.frag"), None).unwrap();

  let cube_vertex_buffer = glium::VertexBuffer::new(&window, &shapes::CUBE).unwrap();
  let sphere_vertex_buffer = glium::VertexBuffer::new(&window, &shapes::sphere(3)).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let basic_params = glium::DrawParameters {
    depth_test: glium::DepthTest::IfLessOrEqual,
    depth_range: (0.0, 1.0),
    depth_write: true,
    backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
    .. Default::default()
  };

  let shapes = vec![
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [1.0, 1.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(3.0, 1.0, -5.0)), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [1.0, 0.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(-3.0, 1.0, -5.0)), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &sphere_vertex_buffer, color: [1.0, 1.0, 1.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 1.0, -5.0)), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &sphere_vertex_buffer, color: [1.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(-6.0, 1.0, -5.0)), shading: shapes::Shading::Flat },
    // Walls
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 1.0, -10.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(21.0, 5.0, 1.0)).into()), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 1.0, 10.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(21.0, 5.0, 1.0)).into()), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(10.0, 1.0, 0.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(1.0, 5.0, 21.0)).into()), shading: shapes::Shading::Smooth },
    shapes::SolidEntity { buffer: &cube_vertex_buffer, color: [0.0, 1.0, 0.0], matrix: cgmath::Matrix4::from_translation(&cgmath::vec3(-10.0, 1.0, 0.0)).mul_m(&cgmath::Matrix3::from_diagonal(&cgmath::vec3(1.0, 5.0, 21.0)).into()), shading: shapes::Shading::Smooth },
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
        camera_change.add_self_v(&cgmath::vec3(-MOVE_SPEED, 0.0, 0.0));
      }
      if keystates.is_scancode_pressed(Scancode::A) {
        camera_change.add_self_v(&cgmath::vec3(MOVE_SPEED, 0.0, 0.0));
      }
      if keystates.is_scancode_pressed(Scancode::W) {
        camera_change.add_self_v(&cgmath::vec3(0.0, 0.0, MOVE_SPEED));
      }
      if keystates.is_scancode_pressed(Scancode::S) {
        camera_change.add_self_v(&cgmath::vec3(0.0, 0.0, -MOVE_SPEED));
      }
      camera_position.add_self_v(&camera_rotation.invert().rotate_vector(&camera_change));
    }

    let mut target = window.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    for shape in &shapes {

      let basic_uniforms = uniform! {
        color: shape.color,
        model: shape.matrix,
        camera: cgmath::Matrix4::from(cgmath::Matrix3::from(camera_rotation)).mul_m(&cgmath::Matrix4::from_translation(&camera_position)),
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

    let fps_text = glium_text::TextDisplay::new(&text_system, &font, &format!("{} fps", fps.average()));

    let fps_text_matrix = cgmath::Matrix4::from_translation(&cgmath::vec3(-1.0, -1.0, 0.0)).mul_m(&cgmath::Matrix4::from(cgmath::Matrix3::from(cgmath::Matrix2::from_value(FONT_SIZE as f32 / HEIGHT as f32))));

    glium_text::draw(&fps_text, &text_system, &mut target, fps_text_matrix.into_fixed(), (1.0, 1.0, 1.0, 1.0));

    target.finish().unwrap();
  }
}
