use firebase::Firebase;
use application::Cache;
use std::fs::File;
use std::io::Read;
const DB_URL: &str = "https://sert-attendance-client.firebaseio.com/";
pub struct DatabaseHandler {
    fb_handle: Firebase
}

impl DatabaseHandler {
    pub fn from_auth_key(key_file_name: &str) -> DatabaseHandler {
        DatabaseHandler {
            fb_handle: Firebase::new(DB_URL)
        }
    }
}

fn retrieve_auth_key(f_name: &str) -> String {
    let mut auth_key_file = File::open(f_name).expect(format!("The authkey file '{}' does not exist", f_name).as_str());
    let mut res = String::new();
    auth_key_file.read_to_string(&mut res);
    res
}