mod web;

fn main() {
    
	println!("Start server");    

	web::server.talk();

    /*
    println!("Guess the number!");
    println!("input guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).ok().expect("Failed to read line");

    println!("You guessed: {}", guess);
	*/
}