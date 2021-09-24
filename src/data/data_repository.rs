use crate::data::*;
use crate::data::trello_data_store::TrelloDataStore;

use async_trait::async_trait;

pub struct DataRepository;

#[async_trait]
pub trait DataStore {
    fn init();
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl DataStore for DataRepository {
    fn init() {
        TrelloDataStore::init();
    }

    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        TrelloDataStore::get_all_boards().await
    }
}