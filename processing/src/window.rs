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
          P.with_borrow_mut(|p| {
            p.mouse_x = position.x as f32;
            p.mouse_y = position.y as f32;
          });
          P.with_borrow(|p| p.mouse_moved());
        }

        GlutinWindowEvent::MouseInput { state, button, ..} => {
          let btn = match button {
            glutin::event::MouseButton::Left => MouseButton::LeftMouseButton,
            glutin::event::MouseButton::Right => MouseButton::RightMouseButton,
            glutin::event::MouseButton::Middle => MouseButton::MiddleMouseButton,
            _ => MouseButton::NoneMouseButton
          };
          
          match state {
            
            glutin::event::ElementState::Pressed => {
              P.with_borrow(|p| p.mouse_pressed(btn));
            },

            glutin::event::ElementState::Released => {
              P.with_borrow(|p| p.mouse_released(btn));
            }
          
          }
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
  PMousePressed,
  PMouseReleased,
  PMouseMoved,
  PMouseWheel,
  PMouseDraged
}


#[repr(C)]
#[derive(Copy, Clone)]
pub enum MouseButton {
  LeftMouseButton, RightMouseButton, MiddleMouseButton, NoneMouseButton
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
  pub fn new (button: MouseButton, event_type: WindowEvents) -> MouseEvent {
    let (x, y) = P.with_borrow(|p| (p.mouse_x, p.mouse_y));
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
  pub mouse: MouseEvent,
  pub keyboard: KeyboardEvent
}

impl WindowEvent {
  pub fn new_mouse_event (button: MouseButton, event_type: WindowEvents) -> WindowEvent {
    WindowEvent {
      mouse: MouseEvent::new(button, event_type)
    }
  }

  pub fn new_keyboard_event (key: u32) -> WindowEvent {
    WindowEvent {
      keyboard: KeyboardEvent {
        key
      }
    }
  }
}