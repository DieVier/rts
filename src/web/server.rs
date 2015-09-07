/**
 *	start server
 */
extern crate hyper;

use std::io::Write;
use self::hyper::Server;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Fresh;

fn hello(_: Request, res: Response<Fresh>) {
	let mut res = res.start().unwrap();
	res.write_all(b"Hello world 123").unwrap();
	res.end().unwrap();
}

/**
 *	START SERVER
 */
pub fn start() {
	Server::http("127.0.0.1:3000").unwrap().handle(hello).unwrap();
}