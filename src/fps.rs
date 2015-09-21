extern crate sdl2;

use ring::Ring;

pub struct FPS {
  ring: Ring,
  timer: sdl2::TimerSubsystem,
}

const SIZE: usize = 11;

pub fn new(timer: sdl2::TimerSubsystem) -> FPS {
  FPS { ring: Ring::new(SIZE), timer: timer }
}

impl FPS {
  pub fn tick(&mut self) {
    let now = self.timer.ticks();
    self.ring.push(now);
  }

  pub fn average(&self) -> u32 {
    let ticks_over_period = self.ring.head() - self.ring.tail();
    ((SIZE - 1) as u32 * 1000) / ticks_over_period
  }
}
