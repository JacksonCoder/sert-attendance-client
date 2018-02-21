#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
mod websocket;


#[get("/")]
fn return_page() -> String {
    "".to_string()
}


fn main() {

}