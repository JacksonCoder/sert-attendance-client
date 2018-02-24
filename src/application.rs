use websocket::WebSocket;
use std::collections::HashMap;
use ws;
use db::DatabaseHandler;
/// Represents the context for the attendance app
/// Locally caches all DB stuff and trigger actions when the conditions are met
pub struct Application<'a> {
    users: Vec<User>,
    trophies: Vec<Trophy>,
    local_user_data: HashMap<String, LocalUserData>,
    cache: Cache,
    comm: WebSocket<'a>,
    db_handle: DatabaseHandler
}

impl<'a> Application<'a> {
    pub fn from_db(db_url: &str, handler: &'a Fn(ws::Message) -> ws::Message) -> Application<'a> {
        Application {
            users: vec![],
            trophies: vec![],
            local_user_data: HashMap::new(),
            cache: Cache {
                users: vec![],
                trophies: vec![],
                local_user_data: HashMap::new()
            },
            comm: WebSocket::new("127.0.0.1:2345", handler),
            db_handle: DatabaseHandler::from_auth_key("some_auth_key.txt")
        }
    }
    pub fn main(&mut self) {
        self.comm.set_cache(self.cache.clone());
        self.comm.start();
        loop {

        }
    }
    fn get_user(&mut self, name: String) -> Option<&User> {
        let mut user_iter = self.users.iter();
        user_iter.find(|&user| user.name == name)
    }
    fn should_update_db(&mut self) -> bool {
        self.users != self.cache.users || self.trophies != self.cache.trophies || self.local_user_data != self.cache.local_user_data
    }
    fn push_to_db(&mut self) {

    }
}
#[derive(Eq, PartialEq, Clone)]
pub struct Cache {
    users: Vec<User>,
    trophies: Vec<Trophy>,
    local_user_data: HashMap<String, LocalUserData>
}
#[derive(Eq, PartialEq, Clone)]
pub struct User {
    name: String,
    student_id: u32,
    trophies: Vec<Trophy>,
    hours: Vec<HourLog>,
    is_mentor: bool
}
#[derive(Eq, PartialEq, Clone)]
pub struct Trophy {
    name: String,
    year: u32,
    place: u8
}
#[derive(Eq, PartialEq, Clone)]
pub struct LocalUserData {
    minutes_since_last_upload: u64,
    is_logged_in: bool,
}
#[derive(Eq, PartialEq, Clone)]
pub struct HourLog {
    event: String,
    hours: u64
}