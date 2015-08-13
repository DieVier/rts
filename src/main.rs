mod web; // import web
mod math;

// starting point
fn main() {
    
	math::test();

	println!("Start server");    
	web::server::start();

}
