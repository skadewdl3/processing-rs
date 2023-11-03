use glium::{ glutin, Display };
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
      
        glutin::event::WindowEvent::CloseRequested => {
          *control_flow = glutin::event_loop::ControlFlow::Exit;
          return;
      
        },
        _ => return,
      
      },
      
      _ => return
    }
  })


}
