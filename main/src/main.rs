use processing::*;
use processing::utils::to_c_str;


extern fn setup () {
	create_window(to_c_str("Hello WOrld"), 800, 800)
}

extern fn draw () {
	println!("From draw");
}

fn main () {
	p_init(setup, draw);
	p_run();
}