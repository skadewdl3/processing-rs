use glium::Display;
use std::cell::RefCell;

pub type Callback = extern fn();

#[derive(Debug, Default)]
pub struct PApplet {
  pub display: Option<Display>,
  pub setup_method: Option<Callback>,
  pub draw_method: Option<Callback>,
}

impl PApplet {
  pub fn draw (&self) {
    if let Some(draw) = self.draw_method {
      draw();
    }
    else {
      println!("No draw method defined");
    }
  }

  pub fn setup (&self) {
    if let Some(setup) = self.setup_method {
      setup();
    }
    else {
      println!("No setup method defined");
    }
  }

  pub fn set_draw (&mut self, draw: Callback) {
    self.draw_method = Some(draw);
  }

  pub fn set_setup (&mut self, setup: Callback) {
    self.setup_method = Some(setup);
  }

  pub fn set_display (&mut self, display: Display) {
    self.display = Some(display);
  }
}

thread_local! {
  pub static P: RefCell<PApplet> = RefCell::new(
    PApplet {
      display: None,
      setup_method: None,
      draw_method: None
    }
  );
}