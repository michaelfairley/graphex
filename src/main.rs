extern crate sdl2;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;

use cgmath::EuclideanVector;
use cgmath::Vector;
use cgmath::Rotation;
use cgmath::Rotation3;
use cgmath::Matrix;

pub mod shapes;

const CAMERA_ROTATE_SPEED: f32 = 1.0/100.0;
const MOVE_SPEED: f32 = 0.1;

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
    .window("Graphex", 1024, 768)
    .resizable()
    .build_glium()
    .unwrap();

  let proj = cgmath::perspective(cgmath::deg(60 as f32), 1024.0/768.0, 1.0, 45.0);

  let light_direction = cgmath::vec3::<f32>(1.0, 2.0, 1.0).normalize();

  let basic_program = glium::Program::from_source(&window, include_str!("shaders/basic.vert"), include_str!("shaders/basic.frag"), None).unwrap();

  let cube_vertex_buffer = glium::VertexBuffer::new(&window, &shapes::CUBE).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let basic_params = glium::DrawParameters {
    depth_test: glium::DepthTest::IfLessOrEqual,
    depth_range: (0.0, 1.0),
    depth_write: true,
    backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
    .. Default::default()
  };


  let mut running = true;

  let mut camera_position = cgmath::vec3::<f32>(0.0, 0.0, 0.0);
  let mut look_up = 0.0f32;
  let mut look_right = 0.0f32;

  let mut events = sdl_context.event_pump().unwrap();

  let mouse = sdl_context.mouse();
  mouse.set_relative_mouse_mode(true);

  while running {
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

    let basic_uniforms = uniform! {
      color: [0.9f32, 0.9, 0.9],
      model: cgmath::Matrix4::from_translation(&cgmath::vec3::<f32>(5.0, 5.0, -10.0)),
      camera: cgmath::Matrix4::from(cgmath::Matrix3::from(camera_rotation)).mul_m(&cgmath::Matrix4::from_translation(&camera_position)),
      proj: proj,
      ambient_intensity: 0.5f32,
      directional_intensity: 0.5f32,
      light_direction: light_direction,
    };

    target.draw(&cube_vertex_buffer,
                &indices,
                &basic_program,
                &basic_uniforms,
                &basic_params).unwrap();

    target.finish().unwrap();
  }
}
