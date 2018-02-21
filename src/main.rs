#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
mod websocket;
use std::fs::File;
use std::io::Read;
use rocket::response::content::Html;
use std::thread;


#[get("/")]
fn return_page() -> Html<String> {
    let mut index = File::open("index.html").unwrap();
    let mut f_str = String::new();
    index.read_to_string(&mut f_str).unwrap();
    Html(f_str)
}


fn main() {
    let server_thread = thread::spawn( || {
        rocket::ignite().mount("/", routes![return_page]).launch();
    });
    let mut socket = websocket::WebSocket::new("127.0.0.1:1234");
    socket.start();
    loop {}
}