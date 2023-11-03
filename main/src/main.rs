use processing::{core, window, utils, shapes};
use window::create_window;
use core::{vtx, p_init, p_run};
use utils::to_c_str;
use shapes::triangle;

extern fn setup () {
	create_window(to_c_str("Hello WOrld"), 800, 800);
}

extern fn draw () {
	println!("From draw");
	triangle(vtx(0, 0), vtx(100, 100), vtx(400, 400))
}

fn main () {
	p_init(setup, draw);
	p_run();
}