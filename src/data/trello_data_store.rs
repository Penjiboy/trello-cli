use crate::data::*;
use crate::data::data_repository::DataStore;

const URL_BASE: &str = "https://api.trello.com/1";
const PATH_TO_KEY: &str = ".config/developer_api_key.txt";
const PATH_TO_TOKEN: &str = ".config/developer_api_token.txt";


pub struct TrelloDataStore;

impl DataStore for TrelloDataStore {
    fn init() {

    }
}