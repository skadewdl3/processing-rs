use glium::Surface;
use crate::core::{Vertex, P};

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