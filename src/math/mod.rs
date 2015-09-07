// mod
extern crate time;
extern crate simd;

pub fn test() {

	println!("test math");
}

static N : i32 = 1000000;

//pub fn simd() {}



struct Vec2D {
    x: i32,
    y: i32
}

pub fn single() {
	let start = time::get_time();
	for x in 0..N {
		println!("{}", x);
	}
	let end = time::get_time();
	let diff = end - start;
	println!("secs:{}", diff.num_milliseconds());
}