use glium::{glutin, Display};
use std::{ffi::CStr, os::raw::c_char};

type Callback = extern fn();
extern fn empty_callback () {}
static mut DRAW_CALLBACK: Callback = empty_callback;


unsafe fn sanitize_c_str (string: *const c_char) -> String {
  let c_str = CStr::from_ptr(string);
  let str_slice = c_str.to_str().unwrap();
  String::from(str_slice)
}

pub fn to_c_str (string: &str) -> *const c_char {
  let c_str = std::ffi::CString::new(string).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  ptr
}


#[no_mangle]
pub extern fn create_window (name: *const c_char, width: u32, height: u32) {

  let event_loop = glutin::event_loop::EventLoop::new();
  
  
  let window_name = unsafe { sanitize_c_str(name) };   

  let wb = glutin::window::WindowBuilder::new()
          .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
          .with_title(window_name);
        
  let cb = glutin::ContextBuilder::new();
  let _display = Display::new(wb, cb, &event_loop).unwrap();

  event_loop.run(move |event, _, control_flow| {

    unsafe {
      DRAW_CALLBACK();
    }

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
    DRAW_CALLBACK = draw;
  }
  setup();
}