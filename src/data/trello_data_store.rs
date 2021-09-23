use std::fs::File;
use std::io::Read;

use crate::data::data_repository::DataStore;
use crate::data::*;

const URL_BASE: &str = "https://api.trello.com/1";
const PATH_TO_KEY: &str = ".config/developer_api_key.txt";
const PATH_TO_TOKEN: &str = ".config/developer_api_token.txt";

lazy_static! {
    #[derive(Debug)]
    static ref KEY: String = {
        let mut key_file = File::open(PATH_TO_KEY).unwrap();
        let mut key = String::from("");
        key_file.read_to_string(&mut key).unwrap();
        key
    };

    #[derive(Debug)]
    static ref TOKEN: String = {
        let mut token_file = File::open(PATH_TO_TOKEN).unwrap();
        let mut token = String::from("");
        token_file.read_to_string(&mut token).unwrap();
        token
    };
}

pub struct TrelloDataStore;

impl DataStore for TrelloDataStore {
    fn init() {
        println!("Key: {:?}\nToken: {:?}", *KEY, *TOKEN);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        TrelloDataStore::init();
        
        println!("Key: {:?}\nToken: {:?}", *KEY, *TOKEN);
        assert_ne!(*KEY, String::from(""));
        assert_ne!(*TOKEN, String::from(""));
    }
}