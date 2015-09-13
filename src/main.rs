extern crate sdl2;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;

use cgmath::EuclideanVector;

pub mod shapes;

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

  let proj = cgmath::perspective(cgmath::deg(90 as f32), 1024.0/768.0, 1.0, 45.0);

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

  while running {
    for event in sdl_context.event_pump().unwrap().poll_iter() {
      use sdl2::event::Event;
      use sdl2::keyboard::Keycode;

      match event {
        Event::Quit {..}
        | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
        => running = false,
        _ => {}
      }
    }

    let mut target = window.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);


    let basic_uniforms = uniform! {
      color: [0.9f32, 0.9, 0.9],
      model: cgmath::Matrix4::from_translation(&cgmath::vec3::<f32>(5.0, 5.0, -10.0)),
      camera: cgmath::Matrix4::from_translation(&cgmath::vec3::<f32>(0.0, 0.0, 0.0)),
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
