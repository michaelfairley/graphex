extern crate sdl2;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;

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
    .window("Graphexs", 1024, 768)
    .resizable()
    .build_glium()
    .unwrap();

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

    target.finish().unwrap();
  }
}
