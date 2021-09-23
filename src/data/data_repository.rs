use crate::data::*;
use crate::data::trello_data_store::TrelloDataStore;

pub struct DataRepository;

pub trait DataStore {
    fn init();
    // fn get_all_boards() -> Vec<Board>;
}