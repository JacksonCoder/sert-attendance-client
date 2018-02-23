extern crate ws;
use self::ws::listen;
use application::Application;
use std::sync::Arc;
use application::Cache;
pub struct WebSocket<'a > {
    ws_path: &'static str,
    on_msg: &'a Fn(ws::Message) -> ws::Message,
    web_cache: Option<Cache>
}

impl<'a> WebSocket<'a> {
    pub fn new(path: &'static str, handler: &'a Fn(ws::Message) -> ws::Message) -> WebSocket<'a> {
        WebSocket {
            ws_path: path,
            on_msg: handler,
            web_cache: None
        }
    }
    pub fn start(&mut self) {
        let handler_copy = self.on_msg;
        listen(self.ws_path,
        |out| {
            move |msg| {
                println!("Server got {}",msg);
                out.send((handler_copy)(msg))
            }
        }).unwrap();
    }
    pub fn set_cache(&mut self, cache: Cache) {
        self.web_cache = Some(cache);
    }
}