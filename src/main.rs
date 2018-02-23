#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate ws;
extern crate firebase;
mod websocket;
mod application;
use std::fs::File;
use std::io::Read;
use rocket::response::content::Html;
use std::thread;
use application::Application;


#[get("/")]
fn return_page() -> Html<String> {
    let mut index = File::open("index.html").unwrap();
    let mut f_str = String::new();
    index.read_to_string(&mut f_str).unwrap();
    Html(f_str)
}


fn main() {
    let handler = |msg: ws::Message| {
        let msg2 = msg.clone();
        let words = match msg {
            ws::Message::Text(ref some_string) => Some(some_string.split_whitespace()),
            _ => None
        };
        match words {
            None => ws::Message::from(""),
            _ => msg2
        }
    };
    let server_thread = thread::spawn( || {
        rocket::ignite().mount("/", routes![return_page]).launch();
    });
    let mut app = Application::from_db("", &handler);
    app.main();
}