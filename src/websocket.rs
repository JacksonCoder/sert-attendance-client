extern crate ws;
use self::ws::listen;
pub struct WebSocket {
    ws_path: &'static str
}

impl WebSocket {
    pub fn new(path: &'static str) -> WebSocket {
        WebSocket {
            ws_path: path
        }
    }
    pub fn start(&mut self) {
        listen(self.ws_path,
        |out| {
            move |msg| {
                println!("Server got {}",msg);
                out.send("")
            }
        }).unwrap();
    }
}