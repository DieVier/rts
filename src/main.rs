mod web; // import web
mod math;
 
fn main() {
    
	math::test();

	println!("Start server");    
	web::server::start();

}