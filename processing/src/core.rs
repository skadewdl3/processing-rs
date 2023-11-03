use glium::{Display, implement_vertex};
use std::cell::RefCell;

/* Type for a callback function received from FFI
 * 
 * Purpose: Defines a callback function type. This is used to define functions
 *  which will be called for drawing or mouse/keyboard/etc events.
 * 
 * Example:
 * 
 * -----------------------------------------------
 * main.c
 * -----------------------------------------------
 * #include <processing.h>
 * 
 * 
 * void setup () {
 *   printf("This is the setup method\n");
 * }
 * 
 * void draw () {
 *   printf("This is the draw method\n");
 * }
 * 
 * int main () {
 *  p_init(setup, draw);
 *  p_run();
 *  return 0;
 * }
 * -----------------------------------------------
 * 
 * 
 * The setup and draw functions received by p_init in Rust
 * are of type Callback.
 */
pub type Callback = extern fn();


/*
 * Purpose: Defines the global state for the processing window
 * 
 * Fields:
 * display: Option<Display> - the glutin display
 * setup_method: Option<Callback> - the setup method which runs once at the beginning
 * draw_method: Option<Callback> - the draw method which runs every frame
 * 
 * Example:
 * let mut p = PApplet::default();
 * 
 */
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

/*
 * Vertex struct for glium
 * Purpose: Defines a vertex struct for glium
 * 
 * Fields:
 * position: [i32; 2] - the position of the vertex (x, y) coordinates
 * TODO: add a z coordinate later
 * 
 * Example:
 * let v = Vertex {
 *   position: [0, 0]
 * };
 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
  position: [i32; 2]
}

impl Vertex {
  fn new (x: i32, y: i32) -> Vertex {
    Vertex {
      position: [x, y]
    }
  }
}

/* Helper function for a shorter syntax to Vertex::new function
 * 
 * Purpose: Creates a vertex struct 
 * 
 * Input Arguments:
 *  x: i32 - x coordinate
 *  y: i32 - y coordinate
 * 
 * Returns:
 *  Vertex - a vertex struct
 * 
 * Example:
 *  let v = vtx(0, 0);
 */
pub extern "C" fn vtx (x: i32, y: i32) -> Vertex {
  Vertex::new(x, y)
}

implement_vertex!(Vertex, position);

/* Create a global state object for the main thread.
 * All state like setup/draw methods, glutin display, event handlers, etc.
 * will be stored in this state.
 *
 * 
 * This decision was made as having a global state removed the need
 * to pass around the PApplet to every function call. This makes it
 * easier for beginners to explore the library.
 * 
 * Moreover, it's better to handle the state in Rust rather than in C
 * as this avoids passing around non-FFI compatible types like closures/vectors.
 */
thread_local! {
  pub static P: RefCell<PApplet> = RefCell::new(
    PApplet {
      display: None,
      setup_method: None,
      draw_method: None
    }
  );
}



/* Purpose - Initialize the setup and draw methods in global state
 * 
 * Input Arguments:
 * setup: Callback - the setup method, runs once before draw loop starts
 * draw: Callback - the draw method, runs every frame
 * 
 * Example:
 * -----------------------------------------------
 * main.c
 * -----------------------------------------------
 * #include <processing.h>
 * 
 * 
 * void setup () {
 *   printf("This is the setup method\n");
 * }
 * 
 * void draw () {
 *   printf("This is the draw method\n");
 * }
 * 
 * int main () {
 *  p_init(setup, draw);
 *  p_run();
 *  return 0;
 * }
 * -----------------------------------------------
 * 
 */
#[no_mangle]
pub extern "C" fn p_init (setup: Callback, draw: Callback) {
  P.with(|p| {
    let mut applet = p.borrow_mut();
    applet.set_setup(setup);
    applet.set_draw(draw);
  }); 
}



/* Calls the setup function. This is called once before the draw loop starts.
 * The draw loop will not start unless create_window is called inside setup function.
  * Example (draw loop will NOT start):
 * -----------------------------------------------
 * main.c
 * -----------------------------------------------
 * #include <processing.h>
 * 
 * 
 * void setup () {
 *   printf("This is the setup method\n");
 * }
 * 
 * void draw () {
 *   printf("This is the draw method\n");
 * }
 * 
 * int main () {
 *  p_init(setup, draw);
 *  p_run();
 *  return 0;
 * }
 * -----------------------------------------------
 * Output:
 * No window is created.
 * 
 * Stdout:
 * This is the setup method.
 * -----------------------------------------------
 * 
 * Example (Draw loop WILL start):
 * -----------------------------------------------
 * main.c
 * -----------------------------------------------
 * #include <processing.h>
 * 
 * 
 * void setup () {
 *   create_window("My First Window", 800, 800);
 * }
 * 
 * void draw () {
 *   printf("This is the draw method\n");
 * }
 * 
 * int main () {
 *  p_init(setup, draw);
 *  p_run();
 *  return 0;
 * }
 * 
 * -----------------------------------------------
 * Output:
 * A blank window is created
 * 
 * Stdout:
 * This is the setup method.
 * This is the draw method.
 * This is the draw method.
 * This is the draw method.
 * .
 * .
 * .
 * (draw method is called once every frame until window is closed)
 * -----------------------------------------------
 * 
 */

#[no_mangle]
pub extern "C" fn p_run () {
  let setup = P.with_borrow(|p| p.setup_method).unwrap();
  setup();
}