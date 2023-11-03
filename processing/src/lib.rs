/*
use glium::{ glutin, Display };
use std::os::raw::c_char;
pub mod utils;


type Callback = extern fn();
type Setter = extern fn(Callback);
extern fn empty_callback () {}


#[repr(C)]
pub enum PAppletOptions {
  MouseOver
}

#[derive(Debug)]
struct Globals {
  pub display: Option<Display>,
  setup_method: Callback,
  draw_method: Callback,
}


#[derive(Debug)]
#[repr(C)]  
pub struct PApplet {
  pub display: Option<Display>,
  setup_method: Callback,
  draw_method: Callback,
  mouse_over: Option<Callback>,
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
pub extern fn p_init (setup: Callback, draw: Callback) -> PApplet {
  let applet = PApplet {
    display: None,
    setup_method: setup,
    draw_method: draw,
    mouse_over: None,
  };
  applet
}

pub extern fn p_run () {
  unsafe {
    GLOBALS.setup();
  }
} 

#[no_mangle]
pub extern fn background () {

}
*/

use glium::{ glutin, Display, implement_vertex, Surface };
use std::os::raw::c_char;
pub mod utils;
pub mod core;
use core::{Callback, P};



#[no_mangle]
pub extern "C" fn p_init (setup: Callback, draw: Callback) {
  P.with(|p| {
    let mut applet = p.borrow_mut();
    applet.set_setup(setup);
    applet.set_draw(draw);
  }); 
}

pub extern "C" fn p_run () {
  let setup = P.with_borrow(|p| p.setup_method).unwrap();
  setup();
  }



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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
  position: [i32; 2]
}

implement_vertex!(Vertex, position);

pub extern "C" fn vtx (x: i32, y: i32) -> Vertex {
  Vertex {
    position: [x, y]
  }
}

#[no_mangle]
pub extern "C" fn triangle (v1: Vertex, v2: Vertex, v3: Vertex) {
  let shape = vec![v1, v2, v3];
  P.with_borrow(|p| {

    let display = p.display.as_ref().unwrap();
    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
  
  
  
    let vertex_shader_src = r#"
      #version 140
      in vec2 position;

      void main() {
          gl_Position = vec4(position, 0.0, 1.0);
      }
    "#;

    let fragment_shader_src = r#"
      #version 140
      out vec4 color;

      void main() {
          color = vec4(1.0, 0.0, 0.0, 1.0);
      }
    "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
  
  
    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
    target.finish().unwrap();
    
  
  
  
  
  });
}