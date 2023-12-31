use processing::{core::{self, p_on}, window::{self, WindowEvents, WindowEvent}, utils, shapes};
use window::create_window;
use core::{vtx, p_init, p_run};
use utils::to_c_str;
use shapes::triangle;

extern fn setup () {
	create_window(to_c_str("Hello WOrld"), 800, 800);
}

extern fn draw () {
	triangle(vtx(0, 0), vtx(100, 100), vtx(400, 400))
}

extern fn test (e: WindowEvent) {
	unsafe {
		println!("mouse clicked at: ({}, {})", e.mouse.x, e.mouse.y);
	}
}

fn main () {
	p_init(setup, draw);
	p_on(WindowEvents::PMousePressed, test);
	p_run();
}