use glium::{ glutin, Display };
use glutin::event::WindowEvent as GlutinWindowEvent;
use std::os::raw::c_char;
use crate::utils;
use crate::core::P;

#[no_mangle]
pub extern "C" fn create_window (name: *const c_char, width: u32, height: u32) {

  let window_name = unsafe { utils::sanitize_c_str(name) };   
  

  let event_loop = glutin::event_loop::EventLoop::new();
  let wb = glutin::window::WindowBuilder::new()
          .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
          .with_title(window_name);      
  let cb = glutin::ContextBuilder::new();
  


  let display = Display::new(wb, cb, &event_loop).unwrap();

  
  {
    P.with_borrow_mut(|p| p.set_display(display));
  }
  
  event_loop.run(move |event, _, control_flow| {

    {
      P.with_borrow(|p| p.draw());
    }
    


    match event {
      glutin::event::Event::WindowEvent { event, .. } => match event {
      
        GlutinWindowEvent::CloseRequested => {
          *control_flow = glutin::event_loop::ControlFlow::Exit;
          return;
      
        },

        GlutinWindowEvent::CursorMoved { position, .. } => {
          P.with_borrow_mut(|p| p.mouse_moved(position.x as f32, position.y as f32));
        }
        _ => return,
      
      },
      
      _ => return
    }
  })


}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum WindowEvents {
  POnClick,
  POnMousePressed,
  POnMouseReleased,
  POnMouseMoved,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub enum MouseButton {
  Left, Right, Middle, None
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MouseEvent {
  pub x: f32,
  pub y: f32,
  pub button: MouseButton,
  pub event_type: WindowEvents
}

impl MouseEvent {
  pub fn new (x: f32, y: f32, button: MouseButton, event_type: WindowEvents) -> MouseEvent {
    MouseEvent {
      x, y, button, event_type
    }
  } 
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyboardEvent {
  pub key: u32
}

#[repr(C)]
pub union WindowEvent {
  pub mouse_event: MouseEvent,
  pub keyboard_event: KeyboardEvent
}