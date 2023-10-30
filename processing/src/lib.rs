use glium::{ glutin, Display };
use std::os::raw::c_char;
pub mod utils;


type Callback = extern fn();
extern fn empty_callback () {}


#[derive(Debug)]
struct Globals {
  pub display: Option<Display>,
  setup_method: Callback,
  draw_method: Callback,
}

static mut GLOBALS: Globals = Globals {
  display: None,
  setup_method: empty_callback,
  draw_method: empty_callback,
};

impl Globals {
  pub fn draw (&self) {
    (self.draw_method)();
  }

  pub fn setup (&self) {
    (self.setup_method)();
  }

  pub fn set_draw (&mut self, draw: Callback) {
    self.draw_method = draw;
  }

  pub fn set_setup (&mut self, setup: Callback) {
    self.setup_method = setup;
  }

  pub fn set_display (&mut self, display: Display) {
    self.display = Some(display);
  }
}


#[no_mangle]
pub extern fn create_window (name: *const c_char, width: u32, height: u32) {

  let window_name = unsafe { utils::sanitize_c_str(name) };   
  

  let event_loop = glutin::event_loop::EventLoop::new();
  let wb = glutin::window::WindowBuilder::new()
          .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
          .with_title(window_name);      
  let cb = glutin::ContextBuilder::new();
  


  let display = Display::new(wb, cb, &event_loop).unwrap();
  unsafe { GLOBALS.set_display(display) }



  event_loop.run(move |event, _, control_flow| {

    unsafe { GLOBALS.draw() }

    match event {
      glutin::event::Event::WindowEvent { event, .. } => match event {
      
        glutin::event::WindowEvent::CloseRequested => {
          *control_flow = glutin::event_loop::ControlFlow::Exit;
          return;
      
        },
        _ => return,
      
      },
      
      _ => return
    }
  });
}


#[no_mangle]
pub extern fn p_init (setup: Callback, draw: Callback) {
  unsafe {
    GLOBALS.set_draw(draw);
    GLOBALS.set_setup(setup);
    GLOBALS.setup();
  }
}

#[no_mangle]
pub extern fn background () {

}